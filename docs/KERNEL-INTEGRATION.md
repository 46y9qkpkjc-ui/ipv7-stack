# IPv7 Kernel Integration Guide

This guide covers integrating the IPv7 protocol implementation with the Linux kernel networking stack.

## Architecture Overview

### Integration Strategy

The IPv7 stack integrates with Linux at three levels:

1. **Userspace Router** (Current - ipv7-stack)
2. **Kernel Module** (In-Kernel driver)
3. **eBPF Programs** (XDP/TC hooks)

### Current Status: v0.1.0

- ✓ Userspace reference implementation complete
- ⧐ Kernel module integration planned for v0.3.0
- ⧐ eBPF integration planned for v0.4.0

## Kernel Module Development

### Prerequisites

```bash
# Install Linux headers
sudo apt-get install linux-headers-$(uname -r)

# Install kernel build tools
sudo apt-get install build-essential git

# Rust for Linux support
rustup install --force-non-host $(rustc --print sysroot | xargs basename)
```

### Module Structure

```
ipv7-kernel/
├── Makefile
├── Kconfig
├── ipv7_main.c          # Module initialization
├── ipv7_packet.c        # Packet handling
├── ipv7_route.c         # Routing logic
├── ipv7_netlink.c       # Netlink interface
├── ipv7_header.h        # Header definitions
└── ipv7_debug.h         # Debug utilities
```

### Basic Module Template

```c
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/netdevice.h>
#include <linux/skbuff.h>

MODULE_LICENSE("MIT");
MODULE_AUTHOR("Arunkumar Subbiah");
MODULE_DESCRIPTION("IPv7 Protocol Kernel Module");
MODULE_VERSION("0.3.0");

static int __init ipv7_module_init(void)
{
    printk(KERN_INFO "IPv7 module loaded\n");
    return 0;
}

static void __exit ipv7_module_exit(void)
{
    printk(KERN_INFO "IPv7 module unloaded\n");
}

module_init(ipv7_module_init);
module_exit(ipv7_module_exit);
```

### Build Module

```bash
cd ipv7-kernel
make
sudo insmod ipv7.ko
lsmod | grep ipv7
```

## eBPF Integration

### eBPF Programs Overview

Three types of eBPF programs for IPv7:

1. **XDP Programs** - Driver/NIC level packet processing
2. **TC Programs** - Traffic control at network interface
3. **Socket Filters** - Application-level packet filtering

### XDP Program Template

```c
#include <linux/bpf.h>
#include <linux/in.h>
#include <linux/ip.h>
#include <bpf/bpf_helpers.h>

SEC("xdp")
int ipv7_xdp_prog(struct xdp_md *ctx)
{
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;
    
    // IPv7 packet processing
    // Return XDP_PASS or XDP_DROP
    return XDP_PASS;
}

char LICENSE[] SEC("license") = "MIT";
```

### Loading eBPF Programs

```bash
# Compile to object file
clang -O2 -target bpf -c ipv7_xdp.c -o ipv7_xdp.o

# Load with ip command
sudo ip link set dev eth0 xdp obj ipv7_xdp.o

# Verify loaded
ip link show eth0

# Unload
sudo ip link set dev eth0 xdp off
```

## Netlink Interface

### Configuration Via Netlink

Design netlink interface for IPv7 configuration:

```c
// Netlink commands
enum {
    IPV7_CMD_ADD_PROVIDER,
    IPV7_CMD_DEL_PROVIDER,
    IPV7_CMD_SET_POLICY,
    IPV7_CMD_GET_STATS,
};

// Example: Configure provider
struct ipv7_provider_msg {
    char provider_id[64];
    unsigned char public_key[32];
};
```

### Userspace Configuration

```bash
# Using nl-cli utility
nl-cli ipv7:add-provider \
    --provider-id "exampleisp" \
    --public-key "abcdef123456"

# Get router statistics
nl-cli ipv7:get-stats
```

## Network Device Driver Integration

### Hook Points

Key integration points in Linux network stack:

```
eth0 (Physical Device)
    ↓
XDP Programs (ipv7_xdp)
    ↓
Driver RX
    ↓
IPv4/IPv6 Protocol Handlers
    ↓
IPv7 Protocol Handler (ipv7_rx_handler)
    ↓
Routing Subsystem
    ↓
TC Programs (ipv7_tc)
    ↓
Qdisc (Queue Discipline)
    ↓
Driver TX
    ↓
Physical Device
```

### Protocol Handler Registration

```c
// Register IPv7 protocol handler
static struct net_protocol ipv7_protocol = {
    .handler = ipv7_rcv,
    .err_handler = ipv7_err,
    .gso_segment = ipv7_gso_segment,
};

inet_add_protocol(&ipv7_protocol, IPPROTO_IPV7);
```

## Performance Optimization

### Kernel Considerations

1. **Lock Contention**
   - Use per-CPU data structures
   - Minimize lock held time
   - Consider RCU for read-heavy paths

2. **Memory Allocation**
   - Preallocate buffers
   - Use memory pools
   - Avoid dynamic allocation in hot paths

3. **Cache Efficiency**
   - Align critical structures to cache lines
   - Group frequently accessed fields
   - Consider NUMA implications

4. **Interrupt Handling**
   - Use NAPI for packet processing
   - Avoid excessive context switching
   - Profile interrupt frequency

### Benchmark Strategy

```bash
# Packet throughput
iperf3 -c 127.0.0.1 -b 10G

# Latency measurement
ping -c 1000 127.0.0.1 | tail -1

# CPU usage profiling
perf record -a -F 99 ./ipv7-stack router-demo
perf report
```

## Testing Framework

### Unit Testing in Kernel

```c
#include <linux/kunit/test.h>

static void ipv7_header_test(struct kunit *test)
{
    struct ipv7_header *hdr;
    KUNIT_EXPECT_EQ(test, sizeof(*hdr), 40);
}

static struct kunit_case ipv7_test_cases[] = {
    KUNIT_CASE(ipv7_header_test),
    {},
};
```

### Integration Testing

```bash
# Test with virtual network interface
sudo ip link add veth0 type veth peer name veth1

# Load IPv7 module
sudo insmod ipv7.ko

# Send test packets
sudo ./test_ipv7_packets veth0

# Check stats
cat /proc/net/ipv7_stats
```

## Security Considerations

### Kernel Space Security

1. **Input Validation**
   - Validate all netlink messages
   - Check packet boundaries
   - Verify cryptographic signatures

2. **Access Control**
   - Require CAP_NET_ADMIN for configuration
   - Restrict module loading
   - Audit sensitive operations

3. **Memory Safety**
   - Use AddressSanitizer for module testing
   - Check for buffer overflows
   - Validate array indexing

### Privilege Requirements

```bash
# Module operations require root
sudo insmod ipv7.ko
sudo rmmod ipv7

# Netlink operations require CAP_NET_ADMIN
sudo ip link set dev eth0 xdp obj ipv7_xdp.o
```

## Future: Rust in Linux Kernel

### Rust Kernel Support

The Linux kernel now supports Rust modules. Future versions may use:

```rust
#![no_std]

use kernel::prelude::*;

module! {
    type: Ipv7Module,
    name: b"ipv7",
    author: b"Arunkumar Subbiah",
    description: b"IPv7 Kernel Module",
    license: b"MIT",
}

pub struct Ipv7Module;

impl kernel::Module for Ipv7Module {
    fn init(_name: &'static CStr, _module: &'static kernel::ThisModule) -> Result<Self> {
        Ok(Ipv7Module)
    }
}
```

## Integration Roadmap

### Phase 1: Kernel Module (v0.3.0)
- [ ] Basic packet handling
- [ ] SPV validation in kernel
- [ ] Netlink interface
- [ ] Statistics collection

### Phase 2: eBPF Support (v0.4.0)
- [ ] XDP packet filtering
- [ ] TC qdisc integration
- [ ] Socket-level filtering
- [ ] Performance optimization

### Phase 3: Full Integration (v1.0.0)
- [ ] Production kernel module
- [ ] Comprehensive eBPF suite
- [ ] Hardware offload support
- [ ] Stable kernel ABI

## Debugging Kernel Code

### Debug Output

```c
#ifdef DEBUG
#define dprintk(fmt, ...) printk(KERN_DEBUG fmt, ##__VA_ARGS__)
#else
#define dprintk(fmt, ...)
#endif

dprintk("IPv7 packet received\n");
```

### Kernel Debugger (kdb)

```bash
# Enable at compile time
CONFIG_KDB=y

# Break into debugger
echo "dmesg" | nc localhost 9876
```

### Dynamic Tracing

```bash
# Using ftrace
echo "function_graph" > /sys/kernel/debug/tracing/tracer
cat /sys/kernel/debug/tracing/trace

# Using trace-cmd
trace-cmd record -e ipv7:*
trace-cmd report
```

## Resources

- **Linux Kernel Networking**: https://lwn.net/Kernel/LDD3/
- **eBPF Documentation**: https://ebpf.io/
- **Kernel Module Writing**: https://www.kernel.org/doc/html/latest/

---

**Last Updated**: April 26, 2026  
**Target Release**: v0.3.0 (Future)
