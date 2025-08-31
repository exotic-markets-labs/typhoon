## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **11** | 🟥 272 (+261) | 🟩 **11** |
| log | 🟩 117 (+1) | 🟥 376 (+260) | 🟩 **116** |
| create_account | 🟩 **1618** | 🟥 4427 (+2809) | 🟩 1659 (+41) |
| transfer | 🟩 **1292** | 🟥 2957 (+1665) | 🟩 1349 (+57) |
| unchecked_accounts | 🟩 **100** | 🟥 2065 (+1965) | 🟩 101 (+1) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **18304** | 🟥 201984 (+183680) | 🟩 18680 (+376) |
