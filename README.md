# Sineclave — public material

Sineclave is a post-quantum data platform **without a stored key**: no server, backup, administrator
or vendor holds a usable decryption key. It is developed by MARANT, d.o.o., Zagreb, and offered for
strategic acquisition or exclusive licensing — see [sineclave.com](https://sineclave.com).

This repository holds what can be read and checked in public. The core of Sineclave is not open source;
**the full source code is available to qualified buyers and their reviewers under a mutual NDA**
(read-only access, protocol specification, test reports and a live session on the production cluster).

## Contents

| Path | What it is |
|---|---|
| [`docs/Sineclave_Security_Model_Whitepaper.pdf`](docs/Sineclave_Security_Model_Whitepaper.pdf) | Security model: threat model, what each kind of compromise yields, standards, operations, measurements and limitations (13 pages) |
| [`docs/MEASUREMENTS.md`](docs/MEASUREMENTS.md) | How each published figure was obtained, and which figures are models rather than measurements |
| [`SECURITY.md`](SECURITY.md) | How to report a security issue |

Open-source verification tools will be added here as they are released.

## Payments (pilot)

Sineclave is being extended to the payment path: a gateway that brings post-quantum protection to the
link between a bank and its payment processor — without changing cards, terminals, the bank's systems or
its certified PIN devices. Details are shared under NDA.

## Status

| Area | Status |
|---|---|
| Production cluster | live — Rust core on all nodes, post-quantum signatures enabled |
| Independent cryptographic audit | next step — scope and test material prepared; the auditor's summary will be published here |
| Production load test (300,000 users per cluster) | next step — currently a model from measured costs |
| Payments gateway | pilot |
| FIPS 140-3 module validation | not started — algorithms are FIPS 203 / 204 |

Contact: info@marant.hr · [sineclave.com/trust](https://sineclave.com/trust/)
