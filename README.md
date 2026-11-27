# Sineclave — public material

Sineclave is a post-quantum data platform **without a stored key**: no server, backup, administrator
or vendor holds a usable decryption key. It is developed by MARANT, d.o.o., Zagreb, and offered for
strategic acquisition or exclusive licensing — see [sineclave.com](https://sineclave.com).

▶ **[Watch Sineclave in just over four minutes](https://sineclave.com/video/)**

This repository holds what can be read and checked in public. The core of Sineclave is not open source;
**the full source code is available to qualified buyers and their reviewers under a mutual NDA**
(read-only access, protocol specification, test reports and a live session on the production cluster).

## One platform, six pillars

| | Pillar | What it does |
|---|---|---|
| 01 | Keyless storage | Data is encrypted so that no single server, vault or administrator ever holds a usable key |
| 02 | Mobile vault | The same protection on Android, with the vault opened only on the owner's devices |
| 03 | End-to-end chat | Messages and files side by side, between contacts verified by the directory |
| 04 | Sineclave Send | File sharing that stays under the sender's control after it leaves their hands |
| 05 | **Sineclave Trust Node** | Keys, signatures and post-quantum time-stamps for the systems you already run, through KMIP and PKCS#11 |
| 06 | **Sineclave Pay Node** | Post-quantum protection for card payments, with every transaction sealed and auditable — in pilot |

## Try it

Sineclave Trust Node's post-quantum time-stamp service is public. Time-stamp any file and verify the
result with nothing but OpenSSL: [`examples/verify-timestamp/`](examples/verify-timestamp/).

## Contents

| Path | What it is |
|---|---|
| [`docs/Sineclave_Security_Model_Whitepaper.pdf`](docs/Sineclave_Security_Model_Whitepaper.pdf) | Security model: threat model, what each kind of compromise yields, standards, operations, measurements and limitations (13 pages) |
| [`docs/MEASUREMENTS.md`](docs/MEASUREMENTS.md) | How each published figure was obtained, and which figures are models rather than measurements |
| [`docs/STRESS_TESTS.md`](docs/STRESS_TESTS.md) | Fault and saturation tests: nodes made to crash, lie or go silent during operation, and what came back |
| [`benchmarks/`](benchmarks/) | Timings of the post-quantum primitives (ML-KEM, ML-DSA, hybrid TLS 1.3), including the FIPS 140-3 validated module |
| [`tools/pq-bench/`](tools/pq-bench/) | Open-source (Apache-2.0) benchmark program that produced them — build and run it yourself |
| [`examples/verify-timestamp/`](examples/verify-timestamp/) | Get and verify a post-quantum RFC 3161 time-stamp from the public Trust Node, using OpenSSL only |
| [`SECURITY.md`](SECURITY.md) | How to report a security issue |

More open-source verification tools will be added here as they are released.

## Status

| Area | Status |
|---|---|
| Production cluster | live — Rust core on all nodes, post-quantum signatures enabled |
| Sineclave Trust Node | live — KMIP and PKCS#11, post-quantum time-stamps, code signing approved on a phone, public transparency log |
| Sineclave Pay Node | pilot, running against the live cluster; details under NDA |
| Independent cryptographic audit | next step — scope and test material prepared; the auditor's summary will be published here |
| Capacity | more than 300,000 users per cluster on 4 vCPU / 8 GB servers — a model from measured costs; a production load test is the next step |
| Scaling with cores | each server spreads its cryptographic work across all CPU cores, so per-cluster capacity grows with core count |
| FIPS 140-3 | server cryptography, including Sineclave Trust Node, runs on a FIPS 140-3 validated module (AWS-LC, CMVP #5314) in FIPS mode; validation of Sineclave itself as a module: not started |

Contact: info@marant.hr · [sineclave.com/trust](https://sineclave.com/trust/)
