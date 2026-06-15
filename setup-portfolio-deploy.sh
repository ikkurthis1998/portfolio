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
# Root-owned. Loads the image GitHub Actions shipped + (re)starts the portfolio
# stack (app + its own Cloudflare tunnel) from /opt/portfolio.
set -euo pipefail
cd /opt/portfolio
docker load < portfolio.tar.gz
docker compose -f docker-compose.deploy.yml up -d --remove-orphans
docker image prune -f >/dev/null 2>&1 || true
SCRIPT
chmod 0755 /usr/local/sbin/portfolio-deploy
chown root:root /usr/local/sbin/portfolio-deploy

# scoped passwordless sudo for ONLY that script
printf '%s\n' "$DEPLOY_USER ALL=(root) NOPASSWD: /usr/local/sbin/portfolio-deploy" \
  > /etc/sudoers.d/portfolio-deploy
chmod 0440 /etc/sudoers.d/portfolio-deploy
visudo -cf /etc/sudoers.d/portfolio-deploy

echo "OK: portfolio CD deploy path ready (/opt/portfolio, portfolio-deploy, scoped sudo, key authorized)."
echo "Next: drop the .env into /opt/portfolio (TUNNEL_TOKEN + DATABASE_URL)."
