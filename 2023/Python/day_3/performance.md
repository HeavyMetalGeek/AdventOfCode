Some friendly and completely unfair comparison between rust and Python. If the Python solution wins, it means I somehow picked a **significantly** more efficient algorithm.

All tests were done on with hyperfine with three warmup rounds. CPU uses was a Ryzen 5600, 32Gb of ram, and a sata SSD

| Language | Runtime | min ... max | runs |
|----------|---------|-------------|------|
| Python (both) | 492.9ms ± 3.7ms | 487.5ms ... 507.5ms | 100 |
| rust (part 1) | 5.8 ± 0.2ms | 5.3ms ... 6.6ms | 100 |
| rust (part 2) | 6.0 ± 0.2ms | 5.7ms ... 6.8ms | 100 |

> Note: Rust results are inaccurate because they ran **too** fast!