# IPv7 Stack Deployment Guide

This guide covers deploying the IPv7 reference implementation in various environments.

## Table of Contents

1. [Local Development](#local-development)
2. [Production Deployment](#production-deployment)
3. [Docker Deployment](#docker-deployment)
4. [Kubernetes Integration](#kubernetes-integration)
5. [Network Integration](#network-integration)
6. [Monitoring & Observability](#monitoring--observability)

## Local Development

### Prerequisites

- Rust 1.56+ (stable or beta)
- Cargo package manager
- Git

### Setup

```bash
# Clone repository
git clone https://github.com/46y9qkpkjc-ui/ipv7-stack.git
cd ipv7-stack

# Build project
cargo build --release

# Run tests
cargo test --all

# Run examples
cargo run --example packet_parser
cargo run --example spv_validation
cargo run --example router_demo
```

### Development Workflow

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets

# Build documentation
cargo doc --no-deps --open

# Run with verbose logging
RUST_BACKTRACE=1 cargo test -- --nocapture
```

## Production Deployment

### Build Optimization

```bash
# Release build with optimizations
cargo build --release

# Binary location
./target/release/ipv7-stack

# Strip binary (reduce size)
strip ./target/release/ipv7-stack
```

### Configuration

Create `ipv7.toml`:

```toml
[router]
id = "edge_router_1"
listen_port = 8080

[spv]
require_validation = true
min_trust_level = 100
drop_on_failure = true

[reputation]
default_scope = "global"
persistence = "memory"  # or "redis", "postgres"

[logging]
level = "info"
format = "json"
```

### Systemd Service

Create `/etc/systemd/system/ipv7-router.service`:

```ini
[Unit]
Description=IPv7 Router Service
After=network-online.target

[Service]
Type=simple
User=ipv7
Group=ipv7
ExecStart=/opt/ipv7-stack/ipv7-stack router-demo
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable ipv7-router
sudo systemctl start ipv7-router
```

## Docker Deployment

### Dockerfile

```dockerfile
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/ipv7-stack /usr/local/bin/
EXPOSE 8080
CMD ["ipv7-stack"]
```

### Build Docker Image

```bash
docker build -t ipv7-stack:latest .
```

### Run Container

```bash
docker run -d \
  --name ipv7-router \
  -p 8080:8080 \
  -e RUST_LOG=info \
  ipv7-stack:latest
```

### Docker Compose

```yaml
version: '3.8'
services:
  ipv7-router:
    image: ipv7-stack:latest
    container_name: ipv7-router
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
    volumes:
      - ./config:/etc/ipv7
    restart: always

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    depends_on:
      - prometheus
```

## Kubernetes Integration

### Deployment Manifest

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ipv7-router
  namespace: network
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ipv7-router
  template:
    metadata:
      labels:
        app: ipv7-router
    spec:
      containers:
      - name: ipv7-router
        image: ipv7-stack:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
```

### Service Mesh Integration

For Istio/Linkerd integration, add sidecar annotations:

```yaml
spec:
  template:
    metadata:
      annotations:
        sidecar.istio.io/inject: "true"
```

## Network Integration

### Firewall Rules

```bash
# Allow IPv7 traffic on port 8080
sudo ufw allow 8080/tcp

# Allow specific provider traffic
sudo ufw allow from 192.168.1.100 to any port 8080
```

### Network Interface Setup

```bash
# Create virtual interface for IPv7
sudo ip link add ipv7-router type bridge
sudo ip addr add 10.0.0.1/24 dev ipv7-router
sudo ip link set ipv7-router up

# Route traffic through IPv7 stack
sudo ip route add 10.0.0.0/24 dev ipv7-router
```

### Load Balancing

Use HAProxy or NGINX:

```nginx
upstream ipv7_backend {
    server localhost:8080;
    server localhost:8081;
    server localhost:8082;
}

server {
    listen 8000;
    location / {
        proxy_pass http://ipv7_backend;
        proxy_set_header X-IPv7-Router $remote_addr;
    }
}
```

## Monitoring & Observability

### Metrics Collection

Use Prometheus exporter:

```bash
cargo add prometheus
```

Implement metrics export endpoint:

```rust
use prometheus::{Counter, Registry};

let registry = Registry::new();
let packets_processed = Counter::new("ipv7_packets_processed", "Total packets processed")?;
registry.register(Box::new(packets_processed.clone()))?;
```

### Logging

Configure structured logging with `tracing`:

```rust
use tracing::{info, warn, error};

tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .with_writer(std::io::stderr)
    .init();

info!("Router started");
warn!("Trust level below threshold");
error!("SPV validation failed");
```

### Health Checks

Implement health check endpoint:

```bash
curl http://localhost:8080/health

# Response:
# {
#   "status": "healthy",
#   "uptime": 3600,
#   "packets_processed": 15234
# }
```

### Performance Monitoring

```bash
# Monitor resource usage
htop -p $(pgrep ipv7-stack)

# Monitor network traffic
nethogs -d 5

# Check router statistics
ipv7-stack router-demo 2>&1 | grep "Statistics"
```

## Troubleshooting

### Common Issues

**Port Already in Use**
```bash
sudo lsof -i :8080
sudo kill -9 <PID>
```

**Insufficient Permissions**
```bash
# Run with appropriate user/group
sudo usermod -aG ipv7 $USER
```

**High CPU Usage**
- Check router policy settings
- Reduce packet processing pipeline complexity
- Profile with `perf`

### Debug Logging

```bash
RUST_LOG=debug RUST_BACKTRACE=1 cargo run --release
```

## Backup & Recovery

### Configuration Backup

```bash
tar -czf ipv7-backup-$(date +%Y%m%d).tar.gz \
  /etc/ipv7/ \
  /opt/ipv7-stack/
```

### Disaster Recovery

```bash
# Restore from backup
tar -xzf ipv7-backup-20260426.tar.gz -C /

# Verify configuration
ipv7-stack test-spv

# Restart service
sudo systemctl restart ipv7-router
```

## Performance Tuning

### Kernel Parameters

```bash
# Increase network buffer sizes
sudo sysctl -w net.core.rmem_max=134217728
sudo sysctl -w net.core.wmem_max=134217728

# Increase connection limits
sudo sysctl -w net.ipv4.tcp_max_syn_backlog=65536
```

### Router Optimization

1. **Disable unnecessary validation** for trusted networks
2. **Adjust trust thresholds** based on threat model
3. **Use reputation caching** for frequently accessed identities
4. **Profile packet processing** to identify bottlenecks

## Production Checklist

- [ ] Configuration reviewed and tested
- [ ] Firewall rules configured
- [ ] Monitoring/alerting deployed
- [ ] Backup strategy implemented
- [ ] Disaster recovery plan tested
- [ ] Security hardening completed
- [ ] Load balancing configured
- [ ] Health checks validated
- [ ] Logging centralization setup
- [ ] Performance baseline established

---

**Last Updated**: April 26, 2026
