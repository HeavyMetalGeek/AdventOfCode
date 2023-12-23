Some friendly and completely unfair comparison between rust and Python. If the Python solution wins, it means I somehow picked a **significantly** more efficient algorithm.

All tests were done on with hyperfine with three warmup rounds. CPU uses was a Ryzen 5600, 32Gb of ram, and a sata SSD

| Language | Runtime | min ... max | runs |
|----------|---------|-------------|------|
| Python (both) | | | 100 |
| rust (part 1) | 0 ± 0ms | 0ms ... 0ms | 100 |
| rust (part 2) | 0 ± 0ms | 0ms ... 0ms | 100 |
