#!/usr/bin/env bash
# ONE-TIME, run as ROOT on the Hetzner box to set up the portfolio's scoped CD
# deploy path. Mirrors Intelligence's scoped-deploy model — reuses the existing
# 'deploy' user and grants it exactly ONE extra passwordless-sudo command.
#
#   ssh root@46.225.180.240 'bash -s' < setup-portfolio-deploy.sh
#
# It does NOT give 'deploy' docker access — the root-owned portfolio-deploy
# script does the docker work. After this, drop the .env into /opt/portfolio
# (see the deploy README / chat instructions).
set -euo pipefail
[ "$(id -u)" = 0 ] || { echo "run as root" >&2; exit 1; }

DEPLOY_USER=deploy
# Public half of the GitHub Actions deploy key (private half is the
# HETZNER_SSH_KEY repo secret). Rotate by regenerating the pair + re-running.
CD_PUBKEY="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMp+C4G8LJsFqSLQt9/aqtZADK07AshKs//SKF8FpSo+ github-actions-portfolio-deploy"

# /opt/portfolio owned by deploy (for the scp'd compose + image tarball + .env)
install -d -o "$DEPLOY_USER" -g "$DEPLOY_USER" /opt/portfolio

# authorize the portfolio CD key for deploy (append — keeps the Intelligence key)
install -d -m 700 -o "$DEPLOY_USER" -g "$DEPLOY_USER" "/home/$DEPLOY_USER/.ssh"
touch "/home/$DEPLOY_USER/.ssh/authorized_keys"
grep -qF "$CD_PUBKEY" "/home/$DEPLOY_USER/.ssh/authorized_keys" \
  || printf '%s\n' "$CD_PUBKEY" >> "/home/$DEPLOY_USER/.ssh/authorized_keys"
chown "$DEPLOY_USER:$DEPLOY_USER" "/home/$DEPLOY_USER/.ssh/authorized_keys"
chmod 600 "/home/$DEPLOY_USER/.ssh/authorized_keys"

# root-owned deploy script — the ONLY portfolio command 'deploy' may sudo
cat > /usr/local/sbin/portfolio-deploy <<'SCRIPT'
#!/usr/bin/env bash
# Root-owned. Loads the image GitHub Actions shipped and (re)starts the portfolio
# stack with a HEALTH-GATE + automatic ROLLBACK to the previously-running image.
#
# A broken build must not silently replace the live site with no recovery: the
# previous image is preserved as portfolio:previous, the new container must come
# up and SERVE before it is kept, and a failed gate restores the previous image.
# Cleanup is SCOPED (only the portfolio image from two deploys ago) — never a
# daemon-global `docker image prune`, since this box co-hosts the Intelligence stack.
set -euo pipefail
cd /opt/portfolio
COMPOSE="docker compose -f docker-compose.deploy.yml"

# Remember the image two deploys old (for scoped cleanup) and preserve the
# currently-running image as the rollback target.
OLD_PREV_ID=""
if docker image inspect portfolio:previous >/dev/null 2>&1; then
  OLD_PREV_ID=$(docker image inspect portfolio:previous -f '{{.Id}}')
fi
if docker image inspect portfolio:latest >/dev/null 2>&1; then
  docker tag portfolio:latest portfolio:previous
fi

# Load the new build (overwrites portfolio:latest) and bring the stack up.
docker load < portfolio.tar.gz
$COMPOSE up -d --remove-orphans

# Health-gate. The runtime image has no curl and port 3000 is not published to the
# host, so probe from a one-shot sidecar on the portfolio network. ANY HTTP status
# proves the binary booted and is serving; a broken image (won't boot / panics)
# yields no response, which trips the rollback.
gate() {
  for _ in $(seq 1 24); do
    code=$(docker run --rm --network portfolio_default curlimages/curl:latest \
      -s -o /dev/null -w '%{http_code}' --max-time 5 http://my-portfolio:3000/health 2>/dev/null || echo 000)
    [ "$code" != "000" ] && return 0
    sleep 5
  done
  return 1
}

if ! gate; then
  echo "portfolio-deploy: new image FAILED to serve within ~2m; rolling back to portfolio:previous" >&2
  if docker image inspect portfolio:previous >/dev/null 2>&1; then
    docker tag portfolio:previous portfolio:latest
    $COMPOSE up -d --remove-orphans
  fi
  exit 1
fi
echo "portfolio-deploy: new image healthy, promoted."

# Scoped cleanup: remove ONLY the portfolio image from two deploys ago, and only
# if it is now dangling (untagged). Never prune daemon-wide.
if [ -n "$OLD_PREV_ID" ] && [ "$(docker image inspect "$OLD_PREV_ID" -f '{{.RepoTags}}' 2>/dev/null)" = "[]" ]; then
  docker rmi -f "$OLD_PREV_ID" >/dev/null 2>&1 || true
fi
SCRIPT
chmod 0755 /usr/local/sbin/portfolio-deploy
chown root:root /usr/local/sbin/portfolio-deploy

# scoped passwordless sudo for ONLY that script
printf '%s\n' "$DEPLOY_USER ALL=(root) NOPASSWD: /usr/local/sbin/portfolio-deploy" \
  > /etc/sudoers.d/portfolio-deploy
chmod 0440 /etc/sudoers.d/portfolio-deploy
visudo -cf /etc/sudoers.d/portfolio-deploy

echo "OK: portfolio CD deploy path ready (/opt/portfolio, portfolio-deploy, scoped sudo, key authorized)."
echo "Next: drop the .env into /opt/portfolio (TUNNEL_TOKEN + SURREALDB_PASS; data lives in the shared SurrealDB 'portfolio' namespace)."
