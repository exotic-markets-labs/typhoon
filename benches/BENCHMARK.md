## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **11** | 🟥 252 (+241) | 🟩 12 (+1) | 🟩 12 (+1) |
| log | 🟩 **117** | 🟥 356 (+239) | 🟩 **117** | 🟩 **117** |
| create_account | 🟩 1580 (+119) | 🟥 4085 (+2624) | 🟩 **1461** | 🟩 1553 (+92) |
| transfer | 🟩 **1291** | 🟥 2694 (+1403) | 🟩 1301 (+10) | 🟩 1325 (+34) |
| unchecked_accounts | 🟩 **100** | 🟥 1764 (+1664) | 🟩 102 (+2) | 🟩 107 (+7) |
| accounts | 🟩 483 (+116) | 🟥 1890 (+1523) | 🟩 439 (+72) | 🟩 **367** |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 18488 (+2088) | 🟥 218496 (+202096) | 🟩 **16400** | 🟥 147600 (+131200) |
