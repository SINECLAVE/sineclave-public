# sineclave-pq-bench

A small, self-contained benchmark of the standard primitives Sineclave builds on:

| Group | What is measured | Library |
|---|---|---|
| ML-KEM (FIPS 203) | ML-KEM-768 keygen, encapsulation, decapsulation | `fips203` (pure Rust) and AWS-LC |
| ML-DSA (FIPS 204) | ML-DSA-44 and ML-DSA-65 keygen, sign, verify | `fips204` (pure Rust) |
| AEAD | AES-256-GCM, 1 KiB and 64 KiB | AWS-LC |
| Hash | SHA-256 and SHA3-256 over 1 MiB | AWS-LC, `sha3` |
| TLS 1.3 | full handshake with X25519MLKEM768, SECP256R1MLKEM768 (hybrid post-quantum) and classical groups | `rustls` on AWS-LC |

It contains no Sineclave protocol code — only calls to public libraries — so anyone can check what it does.

```
cargo run --release                          # table on screen
cargo run --release -- --json result.json    # also save JSON
cargo run --release -- --quick               # fewer repetitions
cargo run --release --features fips          # FIPS 140-3 validated AWS-LC module, in FIPS mode
```

Method: each operation is run in batches after a warm-up; the reported value is the median time per
operation over the batches, on a single thread. The TLS handshake runs both client and server in one thread,
in memory (no network), with a self-signed ECDSA P-256 certificate.

Licence: Apache-2.0 (this tool only; see [LICENSE](LICENSE)).
