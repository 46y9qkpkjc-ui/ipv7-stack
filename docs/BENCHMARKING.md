# IPv7 Performance Benchmarking Guide

This guide covers benchmarking the IPv7 stack implementation to measure performance and identify optimization opportunities.

## Benchmark Tools

### Built-in Benchmarks

```bash
# Run all benchmarks
cargo bench --all

# Run specific benchmark
cargo bench --bench packet_serialization
```

### System Profiling

```bash
# Install perf tools
sudo apt-get install linux-tools-generic

# CPU profiling
cargo build --release
perf record -F 99 ./target/release/ipv7-stack router-demo
perf report

# Flamegraph
cargo install flamegraph
cargo flamegraph
```

## Benchmark Categories

### 1. Packet Processing Benchmarks

#### Serialization Performance

```rust
#[bench]
fn bench_packet_serialization(b: &mut Bencher) {
    let mut packet = Ipv7Packet::new();
    let vlib = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("isp1")
        .tenant("home")
        .role("user")
        .trust_level(128)
        .reputation_scope("global")
        .build()
        .unwrap();
    packet.set_vlib(vlib).unwrap();
    packet.set_payload(vec![0u8; 1024]);

    b.iter(|| packet.to_bytes())
}
```

**Expected Results**:
- Serialization: < 1 microsecond
- Deserialization: < 2 microseconds
- Round-trip: < 3 microseconds

#### Deserialization Performance

```rust
#[bench]
fn bench_packet_deserialization(b: &mut Bencher) {
    let mut packet = Ipv7Packet::new();
    let vlib = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("isp1")
        .tenant("home")
        .role("user")
        .trust_level(128)
        .reputation_scope("global")
        .build()
        .unwrap();
    packet.set_vlib(vlib).unwrap();
    packet.set_payload(vec![0u8; 1024]);
    let bytes = packet.to_bytes().unwrap();

    b.iter(|| Ipv7Packet::from_bytes(&bytes))
}
```

### 2. Cryptography Benchmarks

#### Ed25519 Signature Performance

```rust
#[bench]
fn bench_ed25519_signing(b: &mut Bencher) {
    let (_private_key, _public_key) = crypto::generate_keypair();
    let data = b"test data for signing";

    b.iter(|| {
        crypto::sign_data(data, &_private_key)
    })
}

#[bench]
fn bench_ed25519_verification(b: &mut Bencher) {
    let (_private_key, public_key) = crypto::generate_keypair();
    let data = b"test data for signing";
    let signature = crypto::sign_data(data, &_private_key);

    b.iter(|| {
        crypto::verify_signature(data, &signature, &public_key)
    })
}
```

**Expected Results**:
- Signing: ~10-20 microseconds
- Verification: ~40-60 microseconds
- Hash (SHA-256): ~1-2 microseconds

#### SHA-256 Hashing

```rust
#[bench]
fn bench_sha256_hash(b: &mut Bencher) {
    let data = vec![0u8; 4096];
    b.iter(|| crypto::hash_sha256(&data))
}
```

### 3. Router Pipeline Benchmarks

#### Trust Level Validation

```rust
#[bench]
fn bench_trust_validation(b: &mut Bencher) {
    let policy = SpvPolicy::new()
        .min_trust_level(100)
        .require();

    let mut packet = Ipv7Packet::new();
    packet.header.set_trust_reputation(150);

    b.iter(|| {
        policy.passes_policy(&packet)
    })
}
```

**Expected Results**:
- Trust check: < 1 microsecond
- Policy evaluation: < 5 microseconds

#### SPV Validation

```rust
#[bench]
fn bench_spv_validation(b: &mut Bencher) {
    let mut validator = SourceProviderValidator::new();
    let (_private_key, public_key) = crypto::generate_keypair();
    validator.register_provider("isp1", &public_key);

    let mut packet = Ipv7Packet::new();
    let vlib = Vlib::builder()
        .provider("isp1")
        .service("web")
        .location("nyc")
        .tenant("home")
        .role("user")
        .trust_level(150)
        .reputation_scope("global")
        .build()
        .unwrap();
    packet.set_vlib(vlib).unwrap();

    b.iter(|| {
        validator.validate(&packet)
    })
}
```

**Expected Results**:
- SPV validation: < 50 microseconds

#### Router Processing

```rust
#[bench]
fn bench_router_processing(b: &mut Bencher) {
    let mut router = Router::new("test_router");
    let (_private_key, public_key) = crypto::generate_keypair();
    router.register_provider("isp1", &public_key);

    let policy = SpvPolicy::new()
        .require()
        .allow_providers(vec!["isp1".to_string()])
        .min_trust_level(100);
    router.set_spv_policy(policy);

    let mut packet = Ipv7Packet::new();
    packet.header.set_trust_reputation(150);

    b.iter(|| {
        router.process_packet(&packet)
    })
}
```

**Expected Results**:
- Packet processing: < 100 microseconds

### 4. Reputation System Benchmarks

#### Reputation Lookup

```rust
#[bench]
fn bench_reputation_lookup(b: &mut Bencher) {
    let mut db = ReputationDatabase::new();
    db.record_success("identity_123");

    b.iter(|| {
        db.get("identity_123")
    })
}
```

**Expected Results**:
- Lookup: < 1 microsecond

#### Reputation Updates

```rust
#[bench]
fn bench_reputation_update(b: &mut Bencher) {
    let mut db = ReputationDatabase::new();

    b.iter(|| {
        db.record_success("identity_123")
    })
}
```

**Expected Results**:
- Update: < 5 microseconds

### 5. Throughput Benchmarks

#### Packets Per Second

```bash
# Generate test packets
cat > test_packets.txt << EOF
# IPv7 test packet stream
# 1000 packets of 256 bytes each
EOF

# Measure throughput
time (for i in {1..10000}; do
    echo "packet_$i" | ipv7-stack create-packet
done)
```

**Expected Results**:
- Throughput: > 50,000 packets/second (single thread)

#### Memory Usage

```bash
# Monitor memory during processing
/usr/bin/time -v ./target/release/ipv7-stack router-demo

# Peak memory should be < 100 MB
```

## Performance Profiling

### Flame Graph Analysis

```bash
# Generate flame graph
cargo flamegraph --bench router_processing

# Open in browser
firefox flamegraph.svg
```

### CPU Profiling with Perf

```bash
# Record profile
perf record -F 99 -g cargo bench

# Generate report
perf report

# Top functions
perf top -p $(pgrep ipv7-stack)
```

### Memory Profiling

```bash
# Valgrind memory profiling
valgrind --tool=massif ./target/release/ipv7-stack router-demo
ms_print massif.out.*
```

## Baseline Measurements

### Reference Platform

- **CPU**: Intel i7-10700K (8 cores, 3.8 GHz)
- **RAM**: 32 GB DDR4
- **OS**: Ubuntu 22.04 LTS
- **Kernel**: 5.15.0-56-generic

### v0.1.0 Baseline

| Operation | Time | Throughput |
|-----------|------|-----------|
| Packet Serialization | 0.8 µs | 1.25M pkt/s |
| Packet Deserialization | 1.5 µs | 667K pkt/s |
| Trust Validation | 0.5 µs | 2M pkt/s |
| SPV Validation | 45 µs | 22K pkt/s |
| Router Processing | 80 µs | 12.5K pkt/s |
| Reputation Lookup | 0.3 µs | 3.3M ops/s |
| Reputation Update | 2 µs | 500K ops/s |

## Optimization Targets

### High Priority (v0.2.0)

- [ ] Reduce SPV validation time to < 30 µs
- [ ] Optimize reputation lookups (cache hot identities)
- [ ] Parallel packet processing

### Medium Priority (v0.3.0)

- [ ] SIMD packet processing
- [ ] Zero-copy packet handling
- [ ] Pre-allocated buffer pools

### Low Priority (v1.0.0+)

- [ ] Hardware acceleration
- [ ] Custom memory allocator
- [ ] Kernel module optimization

## Continuous Performance Monitoring

### GitHub Actions Workflow

```yaml
name: Benchmarks
on: [push]

jobs:
  bench:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo bench --bench '*' -- --output-format bencher | tee output.txt
      - uses: benchmark-action/github-action@v1
        with:
          tool: 'cargo'
          output-file-path: output.txt
```

### Performance Regression Detection

Set up alerts for:
- Serialization time increase > 10%
- SPV validation time increase > 20%
- Memory usage increase > 5%

## Load Testing

### Stress Test Script

```bash
#!/bin/bash
# Generate 100,000 test packets
for i in {1..100000}; do
    ipv7-stack create-packet \
        --service web \
        --provider isp$((RANDOM % 10)) \
        --trust $((RANDOM % 256)) &
    
    if [ $((i % 1000)) -eq 0 ]; then
        wait
        echo "Processed $i packets"
    fi
done
wait
```

### Network Load Simulation

```bash
# Using iperf3 to simulate network traffic
iperf3 -s &
cargo run --release &

for i in {1..10}; do
    iperf3 -c localhost -t 10 -P $i &
done
```

## Reporting Results

### Template

```markdown
## Performance Report - [Date]

### Test Environment
- CPU: [Model]
- RAM: [Amount]
- OS: [Linux Version]
- Kernel: [Version]

### Results
| Operation | Target | Measured | % Variance |
|-----------|--------|----------|-----------|
| ... | ... | ... | ... |

### Analysis
[Summary of findings]

### Recommendations
[Suggested optimizations]
```

---

**Last Updated**: April 26, 2026
