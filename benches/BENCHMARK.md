## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 15 (+3) | 🟥 271 (+259) | 🟩 **12** |
| log | 🟩 **118** | 🟥 375 (+257) | 🟩 119 (+1) |
| create_account | 🟩 **1612** | 🟥 4428 (+2816) | 🟩 1791 (+179) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **16736** | 🟥 192912 (+176176) | 🟩 18496 (+1760) |
