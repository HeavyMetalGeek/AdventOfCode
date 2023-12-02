Some friendly and completely unfair comparison between rust and Python. If the Python solution wins, it means I somehow picked a **significantly** more efficient algorithm.

All tests were done on with hyperfine with three warmup rounds. CPU uses was a Ryzen 5600, 32Gb of ram, and a sata SSD

| Language | Runtime | min ... max | runs |
|----------|---------|-------------|------|
| Python (both) | 78.2ms ± 1.5ms | 76.2ms ... 85.1ms | 100 |
| rust | n/a | n/a | 100 |