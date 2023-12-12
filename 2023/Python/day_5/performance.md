Some friendly and completely unfair comparison between rust and Python. If the Python solution wins, it means I somehow picked a **significantly** more efficient algorithm.

All tests were done on with hyperfine with three warmup rounds. CPU uses was a Ryzen 5600, 32Gb of ram, and a sata SSD

| Language | Runtime | min ... max | runs |
|----------|---------|-------------|------|
| Python (both) | 15.981s ± 0.149s | 15.816s ... 16.158s | 100 |
| rust (part 1) | 5 ± 0.2ms | 4.6ms ... 5.7ms | 100 |
| rust (part 2) | 226.964s ± 7.374s | 219.05s ... 236.968s | 5 |

> Note: I went with recursion for part 2, which absolutely *dominates* the time spent on this solution. I "cheated" and stole someone elses solution (for loop and just cleverly added up the wins) for comparison and it ran < 1 sec. So apperently recursion is the very wrong algorithm for this, or recursion behaves "weird" in Python (spent a huge ammount of time copying values???)