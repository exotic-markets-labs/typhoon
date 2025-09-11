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
| create_account | 🟩 1607 (+57) | 🟥 4085 (+2535) | 🟩 1646 (+96) | 🟩 **1550** |
| transfer | 🟩 **1290** | 🟥 2694 (+1404) | 🟩 1301 (+11) | 🟩 1324 (+34) |
| unchecked_accounts | 🟩 **99** | 🟥 1764 (+1665) | 🟩 101 (+2) | 🟩 105 (+6) |
| accounts | 🟩 482 (+116) | 🟥 1890 (+1524) | 🟩 438 (+72) | 🟩 **366** |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 **18464** | 🟥 218496 (+200032) | 🟩 21208 (+2744) | 🟥 147456 (+128992) |
