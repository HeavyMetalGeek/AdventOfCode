Some friendly and completely unfair comparison between rust and Python. If the Python solution wins, it means I somehow picked a **significantly** more efficient algorithm.

All tests were done on with hyperfine with three warmup rounds. CPU uses was a Ryzen 5600, 32Gb of ram, and a sata SSD

| Language | Runtime | min ... max | runs |
|----------|---------|-------------|------|
| Python (both) | 212.0ms ± 9.7ms | 205.6ms ... 296.9ms | 100 |
| rust (part 1) | 7.7ms ± 0.3ms | 7.3ms ... 8.5ms | 100 |
| rust (part 2) | 8.2ms ± 0.2ms | 7.6ms ... 8.9ms | 100 |

I'm quite pleased with the Python solution! Obviously I expected it to lose, but I would say 200ms is much faster than I expected.