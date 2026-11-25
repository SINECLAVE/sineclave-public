# Benchmarks of the post-quantum primitives

These are timings of the **standard** primitives Sineclave builds on — ML-KEM (FIPS 203), ML-DSA (FIPS 204),
AES-256-GCM, SHA-256 / SHA3-256 and a hybrid post-quantum TLS 1.3 handshake. They are produced by
[`tools/pq-bench`](../tools/pq-bench), an open-source program anyone can build and run, so the numbers can be
reproduced on any machine.

They are **not** Sineclave throughput figures: they contain no Sineclave protocol code. They show what the
building blocks cost, which is the input to the capacity figures in [`docs/MEASUREMENTS.md`](../docs/MEASUREMENTS.md).

## Result: 2 vCPU virtual machine (same size class as a Sineclave production node)

Intel(R) Xeon(R) Processor @ 2.30GHz, 2 logical CPUs, Linux x86-64 · 25 November 2026 · median of 101 batches, single thread.
The FIPS column is the same program built against the FIPS 140-3 validated AWS-LC module
(AWS-LC 3 Cryptographic Module, NIST CMVP certificate #5314) running in FIPS mode; "—" means the row does not use that module.

| Operation | µs per op | ops / s (one core) | µs, FIPS module | Note |
|---|---:|---:|---:|---|
| ML-KEM-768 keygen (fips203, pure Rust) | 42.8 | 23,354 | 44.0 | FIPS 203 |
| ML-KEM-768 encaps (fips203) | 43.5 | 22,972 | 43.9 |  |
| ML-KEM-768 decaps (fips203) | 55.5 | 18,013 | 61.2 |  |
| ML-KEM-768 keygen (AWS-LC) | 9.89 | 101,156 | 48.2 |  |
| ML-KEM-768 encaps (AWS-LC) | 11.0 | 90,823 | 15.6 |  |
| ML-KEM-768 decaps (AWS-LC) | 13.6 | 73,595 | 18.4 |  |
| ML-DSA-44 keygen (fips204) | 97.0 | 10,305 | 96.7 | FIPS 204 |
| ML-DSA-44 sign, 64 B (fips204) | 259.4 | 3,855 | 289.3 | hedged |
| ML-DSA-44 verify (fips204) | 63.8 | 15,678 | 65.4 |  |
| ML-DSA-65 keygen (fips204) | 167.1 | 5,986 | 169.0 | FIPS 204 |
| ML-DSA-65 sign, 64 B (fips204) | 408.9 | 2,446 | 433.1 | hedged |
| ML-DSA-65 verify (fips204) | 108.1 | 9,251 | 103.0 |  |
| AES-256-GCM seal 1 KiB (AWS-LC) | 0.14 | 7,350,239 | 0.13 | 7527 MB/s |
| AES-256-GCM seal 64 KiB (AWS-LC) | 3.86 | 259,380 | 3.79 | 16999 MB/s |
| SHA-256, 1 MiB (AWS-LC) | 545.5 | 1,833 | 534.0 | 1922 MB/s |
| SHA3-256, 1 MiB (sha3, pure Rust) | 2,139 | 468 | 2,132 | 490 MB/s |
| full handshake, X25519MLKEM768 (hybrid PQ) | 191.9 | 5,211 | 250.1 | post-quantum; 3072 B on the wire, both sides on one core |
| full handshake, SECP256R1MLKEM768 (hybrid PQ) | 235.0 | 4,255 | 310.4 | post-quantum; 3138 B on the wire, both sides on one core |
| full handshake, SECP256R1 (classical) | 184.4 | 5,424 | 210.9 | not post-quantum; 866 B on the wire, both sides on one core |
| full handshake, X25519 (classical) | 143.7 | 6,960 | — | not post-quantum; 800 B on the wire, both sides on one core |

Raw output: [`results/`](results/).

## Reading the numbers

- A post-quantum key exchange (ML-KEM-768) costs tens of microseconds; a post-quantum signature (ML-DSA-65)
  well under a millisecond. On one core of a small server that is thousands to tens of thousands of operations
  per second — post-quantum cryptography is not the bottleneck of a protected operation.
- A full hybrid TLS 1.3 handshake (X25519MLKEM768) costs about the same CPU as a classical one; the visible
  difference is size on the wire (~3 KB instead of ~0.8 KB), because ML-KEM keys and ciphertexts are larger.
- In FIPS mode, key generation runs an additional self-check (pairwise consistency test), so ML-KEM key
  generation is slower there; other operations are close to the default build.
- Classical X25519 alone is not an approved FIPS key exchange, so the FIPS build does not offer it.

## Reproduce

```
cd tools/pq-bench
cargo run --release -- --json my-result.json
# FIPS 140-3 validated module (needs CMake, Go and Clang):
cargo run --release --features fips -- --json my-result-fips.json
```

Results from other machines are welcome as pull requests to `benchmarks/results/`.
