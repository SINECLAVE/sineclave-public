# Stress tests: faults injected while the system runs

What a buyer should ask of a "keyless" system is not only whether it is fast, but whether it ever returns a
wrong answer when parts of it fail or lie. These are the results of the fault and saturation tests run on
the release that went to production on 21 November 2026. Protected data is read back and compared, byte
for byte, with what was stored.

**Environment.** A full test cluster — the same node roles, node count and software as production at the
time — on test machines, not on the production cluster. Results below are **measured**.

## Faulty and malicious nodes

Five separate tests. In each, a continuous stream of protected reads runs while one node is made to fail
in one of these ways:

1. it returns **altered content**;
2. it **withholds acknowledgements**;
3. it goes **silent**;
4. it **crashes**;
5. a **key-service node crashes**.

| Across all five tests | Result |
|---|---|
| Protected reads per test | 2,956 – 3,084 |
| Errors | 0 |
| Incorrect results | 0 |
| Latency, 95th percentile | 0.41 s |

## Saturation and volume

| Test | Result |
|---|---|
| 10 minutes at saturation with **malformed input** mixed into the load | 0 errors, 0 incorrect results |
| **60,000 protected objects** stored, with node failures and injected faults | 0 incorrect results |
| Node restart with 60,000 objects | objects reloaded in 1.0 s, using 51 MB of memory |
| Node replaced with an **empty disk** | re-synchronised 60,000 objects from its peers in 14 s |

## Implementation cross-checks

| Check | Result |
|---|---|
| Differential fuzzing of the record format between two independent implementations | 42,000 cases + 36,000 cases + 4,000 mutated records: 0 differences |
| Regression suite, both implementations of the key-service node | 63 / 63 and 53 / 53 passing |

## What stress testing found and changed

Stress testing is only useful if it finds things. One example from this release: a single, valid but very
large request could make a node assemble its whole history in memory. Responses are now capped and paged,
total in-flight memory is bounded, and slow clients that stop reading are disconnected. The fault tests were
then repeated on the fixed release.

## What is not claimed here

- **Capacity.** 300,000 users per cluster is still a **model** built from measured costs, not a load-test
  result; a production load test is the next step (see [MEASUREMENTS.md](MEASUREMENTS.md)).
- **The payments gateway** is in pilot; no figures are published for it yet.
- The test harness and full reports are available to qualified buyers under NDA.

For the cost of the underlying post-quantum primitives, see [`benchmarks/`](../benchmarks/), which anyone
can reproduce with the open-source [`tools/pq-bench`](../tools/pq-bench).
