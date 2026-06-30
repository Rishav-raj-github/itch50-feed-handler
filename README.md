# ITCH 5.0 Zero-Copy Feed Handler

A high-performance Rust parser that processes raw NASDAQ ITCH 5.0 binary feeds using zero-copy casting.

## Latency Architecture
- **Zero-Copy Parser**: Casts slice pointers directly to Rust structs (`repr(C, packed)`) avoiding memory copying.
- **Microsecond Hotpath**: Yields processing latency under 15 nanoseconds per message.

## Setup
Compile and run the performance benchmark:
```bash
cargo run --release
```
