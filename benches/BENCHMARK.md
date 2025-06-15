## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **15** | 🟥 271 (+256) | 🟥 42 (+27) |
| log | 🟩 **118** | 🟥 375 (+257) | 🟩 146 (+28) |
| create_account | 🟩 **1459** | 🟥 4428 (+2969) | 🟩 1911 (+452) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **9712** | 🟥 192912 (+183200) | 🟨 19384 (+9672) |
