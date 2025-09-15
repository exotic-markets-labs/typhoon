## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **11** | 🟥 252 (+241) | 🟩 13 (+2) | 🟩 12 (+1) |
| log | 🟩 **117** | 🟥 356 (+239) | 🟩 118 (+1) | 🟩 **117** |
| create_account | 🟩 1580 (+27) | 🟥 4085 (+2532) | 🟩 1615 (+62) | 🟩 **1553** |
| transfer | 🟩 **1291** | 🟥 2694 (+1403) | 🟩 1303 (+12) | 🟩 1325 (+34) |
| unchecked_accounts | 🟩 **100** | 🟥 1764 (+1664) | 🟩 103 (+3) | 🟩 107 (+7) |
| accounts | 🟩 483 (+116) | 🟥 1890 (+1523) | 🟩 440 (+73) | 🟩 **367** |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 18488 (+2048) | 🟥 218496 (+202056) | 🟩 **16440** | 🟥 147600 (+131160) |
