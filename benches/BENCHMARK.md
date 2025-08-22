## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **10** | 🟥 272 (+262) | 🟩 11 (+1) |
| log | 🟩 **116** | 🟥 376 (+260) | 🟩 **116** |
| create_account | 🟩 **1617** | 🟥 4426 (+2809) | 🟩 1673 (+56) |
| transfer | 🟩 **1291** | 🟥 2957 (+1666) | 🟩 1386 (+95) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **18328** | 🟥 197672 (+179344) | 🟩 18912 (+584) |
