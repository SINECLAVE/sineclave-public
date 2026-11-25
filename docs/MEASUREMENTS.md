# How the published figures were obtained

Every figure Sineclave publishes is labelled as **measured** or **model**. No figures are claimed yet for the payments gateway, which is in pilot.

| Figure | Value | Label | How it was obtained |
|---|---|---|---|
| Correctness under faults | 60,000 vaults, 0 incorrect results | measured | Stress test on a full cluster with nodes crashed, restarted and made to send wrong or conflicting data during operation; every vault was read back and compared with what was stored. |
| Send | ≈2.6 s to send, ≈1.5 s to open | measured | Desktop client against the production cluster, end to end, including encryption, upload and key release. |
| Infrastructure cost | ≈ €100 per month per cluster | measured | Current production hosting: one small virtual server per node, 2 vCPU / 4 GB. |
| Capacity | 300,000 users per cluster | **model** | Per-operation costs (CPU, network, storage) measured on the real system, multiplied out for an assumed usage profile on 2 vCPU / 4 GB nodes. **Not yet a load-test result**; a production load test is the next step and its result will replace this model. |

The full test reports and the test harness are available under NDA.
