## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **12** | 🟥 238 (+226) | 🟩 13 (+1) | 🟩 13 (+1) |
| log | 🟩 **116** | 🟥 342 (+226) | 🟩 117 (+1) | 🟩 117 (+1) |
| create_account | 🟩 1570 (+115) | 🟥 3790 (+2335) | 🟩 **1455** | 🟩 1550 (+95) |
| transfer | 🟩 **1289** | 🟨 2442 (+1153) | 🟩 1299 (+10) | 🟩 1316 (+27) |
| unchecked_accounts | 🟩 **99** | 🟥 1738 (+1639) | 🟩 101 (+2) | 🟩 105 (+6) |
| accounts | 🟩 316 (+25) | 🟥 1711 (+1420) | 🟩 **291** | 🟩 358 (+67) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 17944 (+2680) | 🟥 212560 (+197296) | 🟩 **15264** | 🟥 115264 (+100000) |
