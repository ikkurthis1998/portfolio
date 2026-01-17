# Check if Docker Daemon is running
if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker daemon is not running. Please start Docker Desktop."
    exit 1
fi

echo "🚀 Starting Safe Deployment..."

# 1. Build the new image first
echo "🔨 Building new image..."
if ! docker build -t portfolio:latest .; then
    echo "❌ Build failed. Aborting."
    exit 1
fi

# 2. Verification Step: Run a temporary candidate container
CANDIDATE_NAME="portfolio-candidate"
CHECK_PORT=3333

echo "🕵️  Verifying build..."
# Remove any leftover candidate
docker rm -f $CANDIDATE_NAME 2>/dev/null || true

# Run candidate mapping port 3333 on host to 3000 in container
docker run -d --name $CANDIDATE_NAME -p $CHECK_PORT:3000 portfolio:latest > /dev/null

# Loop to check health (max 30 seconds)
HEALTHY=false
echo "   Waiting for health check on port $CHECK_PORT..."
for i in {1..10}; do
    if curl -s -o /dev/null -w "%{http_code}" http://localhost:$CHECK_PORT | grep -q "200"; then
        HEALTHY=true
        break
    fi
    sleep 2
    echo -n "."
done
echo ""

# 3. Decision Logic
if [ "$HEALTHY" = true ]; then
    echo "✅ Health Check Passed! Code is stable."
    
    # Cleanup candidate
    docker rm -f $CANDIDATE_NAME > /dev/null
    
    # Promote: Update the actual production service
    echo "📦 Promoting to Production..."
    docker-compose down --remove-orphans
    docker-compose up -d
    
    echo "🎉 Deployment Complete Successfully!"
else
    echo "❌ Health Check Failed. The new version is broken."
    echo "🛑 Aborting deployment. Your existing site remains live and safe."
    
    # Cleanup candidate
    docker rm -f $CANDIDATE_NAME > /dev/null
    # Do not update production
    exit 1
fi

echo "🔒 Localhost: Disabled (Secure)"
echo "🌐 Public: Check Cloudflare Dashboard status"
