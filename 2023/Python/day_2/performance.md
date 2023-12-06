Some friendly and completely unfair comparison between rust and Python. If the Python solution wins, it means I somehow picked a **significantly** more efficient algorithm.

All tests were done on with hyperfine with three warmup rounds. CPU uses was a Ryzen 5600, 32Gb of ram, and a sata SSD

| Language | Runtime | min ... max | runs |
|----------|---------|-------------|------|
| Python (both) | 78.2ms ± 1.5ms | 76.2ms ... 85.1ms | 100 |
| rust (part 1) | 5.3 ± 0.8ms | 4.8ms ... 12.5ms | 100 |
| rust (part 2) | 5.2 ± 0.2ms | 4.9ms ... 6.0ms | 100 |

> Note: Rust results are inaccurate because they ran **too** fast!