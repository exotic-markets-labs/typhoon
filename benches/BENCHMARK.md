## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **12** | 🟥 238 (+226) | 🟩 **12** | 🟩 **12** |
| log | 🟩 **116** | 🟥 342 (+226) | 🟩 **116** | 🟩 117 (+1) |
| create_account | 🟩 1570 (+116) | 🟥 3951 (+2497) | 🟩 **1454** | 🟩 1554 (+100) |
| transfer | 🟩 **1289** | 🟥 2603 (+1314) | 🟩 1297 (+8) | 🟩 1325 (+36) |
| unchecked_accounts | 🟩 **99** | 🟥 1738 (+1639) | 🟩 100 (+1) | 🟩 104 (+5) |
| accounts | 🟩 316 (+26) | 🟥 1711 (+1421) | 🟩 **290** | 🟩 357 (+67) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 17944 (+2744) | 🟥 217048 (+201848) | 🟩 **15200** | 🟥 145992 (+130792) |
