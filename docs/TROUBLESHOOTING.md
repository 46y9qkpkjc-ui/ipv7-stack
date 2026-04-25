# IPv7 Stack Troubleshooting Guide

Common issues and solutions for the IPv7 reference implementation.

## Table of Contents

1. [Build Issues](#build-issues)
2. [Runtime Issues](#runtime-issues)
3. [Validation Issues](#validation-issues)
4. [Performance Issues](#performance-issues)
5. [Deployment Issues](#deployment-issues)
6. [FAQ](#faq)

## Build Issues

### Issue: Compilation Error - "error: could not compile"

**Symptoms**:
```
error: could not compile `ipv7-stack`
```

**Solutions**:
1. Update Rust toolchain:
   ```bash
   rustup update
   ```

2. Clean and rebuild:
   ```bash
   cargo clean
   cargo build
   ```

3. Check Rust version requirement:
   ```bash
   rustc --version
   # Should be 1.56+
   ```

### Issue: Dependency Version Conflicts

**Symptoms**:
```
error: version requirements don't match
```

**Solutions**:
1. Update dependencies:
   ```bash
   cargo update
   ```

2. Check lock file:
   ```bash
   cargo update --locked
   ```

3. Review Cargo.toml for conflicting versions:
   ```bash
   cargo tree --duplicates
   ```

### Issue: Ed25519 Signature Verification Errors

**Symptoms**:
```
error: no method named `verify` found for struct `VerifyingKey`
```

**Solutions**:
1. Ensure Verifier trait is imported:
   ```rust
   use ed25519_dalek::Verifier;
   ```

2. Verify ed25519-dalek version in Cargo.toml:
   ```toml
   ed25519-dalek = "2.0"
   ```

3. Clean and rebuild:
   ```bash
   cargo clean
   cargo build
   ```

### Issue: Test Failures During Build

**Symptoms**:
```
test result: FAILED. 0 passed; 1 failed
```

**Solutions**:
1. Run tests verbosely:
   ```bash
   cargo test -- --nocapture
   ```

2. Check for panic messages:
   ```bash
   RUST_BACKTRACE=1 cargo test
   ```

3. Run specific failing test:
   ```bash
   cargo test test_header_serialization -- --nocapture
   ```

## Runtime Issues

### Issue: "Address Already in Use" Error

**Symptoms**:
```
Error: Address already in use (os error 98)
```

**Solutions**:
1. Find and kill process using port:
   ```bash
   sudo lsof -i :8080
   sudo kill -9 <PID>
   ```

2. Use different port:
   ```bash
   LISTEN_PORT=8081 cargo run
   ```

3. Wait for port to be released:
   ```bash
   sleep 60
   cargo run
   ```

### Issue: "Permission Denied" Error

**Symptoms**:
```
Error: Permission denied (os error 13)
```

**Solutions**:
1. Run with appropriate privileges:
   ```bash
   sudo cargo run
   ```

2. Create IPv7 user:
   ```bash
   sudo useradd -r -s /bin/false ipv7
   sudo usermod -aG ipv7 $USER
   ```

3. Set proper file permissions:
   ```bash
   chmod 755 target/release/ipv7-stack
   chmod 644 /etc/ipv7/config.toml
   ```

### Issue: Panic on Malformed Packet

**Symptoms**:
```
thread 'main' panicked at 'index out of bounds'
```

**Solutions**:
1. Enable bounds checking:
   ```bash
   RUST_BACKTRACE=full cargo run
   ```

2. Add input validation:
   ```rust
   if data.len() < 40 {
       return Err(Ipv7Error::InvalidPacket("Too short".into()));
   }
   ```

3. Use Result types properly:
   ```rust
   let packet = Ipv7Packet::from_bytes(data)?;
   ```

## Validation Issues

### Issue: SPV Validation Always Fails

**Symptoms**:
```
SpvValidationFailed: Provider not found
```

**Solutions**:
1. Check provider registration:
   ```bash
   # Verify in code
   validator.register_provider("isp1", &public_key);
   ```

2. Verify public key format:
   ```bash
   # Should be 32 bytes for Ed25519
   assert_eq!(public_key.len(), 32);
   ```

3. Check VLIB provider name:
   ```rust
   let vlib = Vlib::builder()
       .provider("isp1")  // Must match registered name
       .build()?;
   ```

### Issue: Trust Level Always Below Threshold

**Symptoms**:
```
PolicyViolation: Trust level too low (50 < 100)
```

**Solutions**:
1. Verify trust level in packet:
   ```bash
   packet.header.set_trust_reputation(150);
   ```

2. Check policy threshold:
   ```rust
   let policy = SpvPolicy::new()
       .min_trust_level(100);  // Adjust as needed
   ```

3. Inspect VLIB trust level:
   ```rust
   let vlib = Vlib::builder()
       .trust_level(150)  // Must meet minimum
       .build()?;
   ```

### Issue: Signature Verification Fails

**Symptoms**:
```
SignatureVerificationFailed: Invalid signature
```

**Solutions**:
1. Verify signature format:
   ```bash
   # Should be 64 bytes for Ed25519
   assert_eq!(signature.len(), 64);
   ```

2. Ensure data hasn't been modified:
   ```rust
   // Sign original data
   let sig = crypto::sign_data(&original_data, &private_key);
   
   // Verify on same data
   crypto::verify_signature(&original_data, &sig, &public_key)?;
   ```

3. Check key pair correspondence:
   ```rust
   let (private_key, public_key) = crypto::generate_keypair();
   // Use matching pair
   ```

## Performance Issues

### Issue: High CPU Usage

**Symptoms**:
- Router consuming 100% CPU
- Slow packet processing

**Solutions**:
1. Profile with perf:
   ```bash
   perf record -p $(pgrep ipv7-stack)
   perf report
   ```

2. Check reputation database size:
   ```rust
   let rep_db = router.get_reputation_db();
   println!("Identities tracked: {}", rep_db.len());
   ```

3. Reduce policy complexity:
   ```rust
   let policy = SpvPolicy::new()
       .min_trust_level(100);  // Simpler policies
   ```

4. Disable unnecessary validation:
   ```rust
   // For trusted networks
   let policy = SpvPolicy::new()
       .require_spv(false);
   ```

### Issue: High Memory Usage

**Symptoms**:
- Memory grows over time
- OOM killer invoked

**Solutions**:
1. Check reputation database:
   ```bash
   # Memory usage per identity
   rep_db.get_stats()  // If implemented
   ```

2. Limit number of tracked identities:
   ```rust
   // Implement expiration policy
   if rep_db.len() > MAX_IDENTITIES {
       rep_db.expire_old_entries();
   }
   ```

3. Monitor memory:
   ```bash
   /usr/bin/time -v ./target/release/ipv7-stack
   ```

### Issue: Slow Packet Parsing

**Symptoms**:
- Deserialization takes > 10 µs
- Latency issues

**Solutions**:
1. Profile deserialization:
   ```bash
   cargo bench --bench packet_deserialization
   ```

2. Avoid cloning:
   ```rust
   // Use references
   fn parse(&self, data: &[u8]) -> Result<()> {
       // Don't clone data unnecessarily
   }
   ```

3. Use zero-copy approaches:
   ```rust
   // Return views into data, not copies
   ```

## Deployment Issues

### Issue: Docker Container Won't Start

**Symptoms**:
```
container exited with code 1
```

**Solutions**:
1. Check container logs:
   ```bash
   docker logs ipv7-router
   ```

2. Verify entrypoint:
   ```dockerfile
   CMD ["ipv7-stack"]  # Ensure binary exists
   ```

3. Test locally first:
   ```bash
   cargo build --release
   ./target/release/ipv7-stack --version
   ```

### Issue: Kubernetes Pod CrashLoopBackOff

**Symptoms**:
```
CrashLoopBackOff - Container restarting repeatedly
```

**Solutions**:
1. Check pod logs:
   ```bash
   kubectl logs -f deployment/ipv7-router
   ```

2. Describe pod:
   ```bash
   kubectl describe pod <pod-name>
   ```

3. Test deployment locally:
   ```bash
   kubectl apply -f deployment.yaml
   kubectl get pods
   ```

4. Check resource limits:
   ```yaml
   resources:
     requests:
       memory: "256Mi"
       cpu: "250m"
   ```

### Issue: Health Check Failures

**Symptoms**:
```
readinessProbe failed
```

**Solutions**:
1. Implement health endpoint:
   ```rust
   // Return 200 OK for /health
   ```

2. Check probe configuration:
   ```yaml
   readinessProbe:
     httpGet:
       path: /health
       port: 8080
     initialDelaySeconds: 5
     periodSeconds: 5
   ```

3. Test health endpoint:
   ```bash
   curl http://localhost:8080/health
   ```

## FAQ

### Q: How do I reset the reputation database?

A:
```rust
let mut router = Router::new("router");
let fresh_db = ReputationDatabase::new();
router.reputation_db = fresh_db;
```

### Q: Can I use IPv7 with existing IPv4/IPv6?

A: Yes. IPv7 is designed to coexist with IPv4/IPv6. Use:
- Different port numbers
- Virtual network interfaces
- eBPF for selective forwarding

### Q: How do I debug packet parsing?

A:
```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Parse specific hex
cargo run -- parse-packet --hex "70000000..."
```

### Q: What's the maximum packet size?

A: 65,535 bytes (limited by payload_length field in header)

### Q: How do I implement persistence?

A: Replace in-memory ReputationDatabase with:
- Redis for caching
- PostgreSQL for permanent storage
- RocksDB for embedded persistence

### Q: Can I run multiple routers?

A:
```rust
let router1 = Router::new("router1");
let router2 = Router::new("router2");

// Independent reputation databases
```

---

**Last Updated**: April 26, 2026

For additional help, please:
1. Check [RESOURCES.md](../RESOURCES.md)
2. Review [IMPLEMENTATION-GUIDE.md](../IMPLEMENTATION-GUIDE.md)
3. Open an issue on [GitHub](https://github.com/46y9qkpkjc-ui/ipv7-stack/issues)
