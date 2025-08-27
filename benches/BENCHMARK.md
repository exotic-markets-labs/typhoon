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
| create_account | 🟩 **1619** | 🟥 4427 (+2808) | 🟩 1673 (+54) |
| transfer | 🟩 **1293** | 🟥 2957 (+1664) | 🟩 1386 (+93) |
| unchecked_accounts | 🟩 **101** | 🟥 2065 (+1964) | 🟩 102 (+1) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **18400** | 🟥 201984 (+183584) | 🟩 18936 (+536) |
