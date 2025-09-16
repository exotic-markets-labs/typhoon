## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **11** | 🟥 252 (+241) | 🟩 **11** | 🟩 12 (+1) |
| log | 🟩 117 (+1) | 🟥 356 (+240) | 🟩 **116** | 🟩 117 (+1) |
| create_account | 🟩 1580 (+122) | 🟥 4085 (+2627) | 🟩 **1458** | 🟩 1553 (+95) |
| transfer | 🟩 **1291** | 🟥 2694 (+1403) | 🟩 1300 (+9) | 🟩 1325 (+34) |
| unchecked_accounts | 🟩 **100** | 🟥 1764 (+1664) | 🟩 101 (+1) | 🟩 107 (+7) |
| accounts | 🟨 483 (+166) | 🟥 1890 (+1573) | 🟩 **317** | 🟩 367 (+50) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 18488 (+3360) | 🟥 218496 (+203368) | 🟩 **15128** | 🟥 147600 (+132472) |
