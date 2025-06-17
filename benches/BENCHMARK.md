## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **11** | 🟥 272 (+261) | 🟩 12 (+1) |
| log | 🟩 **117** | 🟥 376 (+259) | 🟩 **117** |
| transfer | 🟩 **1611** | 🟥 4426 (+2815) | 🟩 1803 (+192) |
| create_account | 🟩 **1446** | 🟥 2957 (+1511) | 🟩 1524 (+78) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **17552** | 🟥 197672 (+180120) | 🟩 19368 (+1816) |
