## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **12** | 🟥 238 (+226) | 🟩 **12** | 🟩 13 (+1) |
| log | 🟩 **116** | 🟥 342 (+226) | 🟩 **116** | 🟩 117 (+1) |
| create_account | 🟩 1570 (+116) | 🟥 3951 (+2497) | 🟩 **1454** | 🟩 1550 (+96) |
| transfer | 🟩 **1289** | 🟥 2603 (+1314) | 🟩 1297 (+8) | 🟩 1316 (+27) |
| unchecked_accounts | 🟩 **99** | 🟥 1738 (+1639) | 🟩 100 (+1) | 🟩 105 (+6) |
| accounts | 🟩 316 (+26) | 🟥 1711 (+1421) | 🟩 **290** | 🟩 358 (+68) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 17944 (+2744) | 🟥 217048 (+201848) | 🟩 **15200** | 🟥 115264 (+100064) |
