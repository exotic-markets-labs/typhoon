## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **11** | 🟥 180 (+169) | 🟩 12 (+1) |
| log | 🟩 **117** | 🟥 284 (+167) | 🟩 **117** |
| transfer | 🟩 **1605** | 🟥 4444 (+2839) | 🟩 1666 (+61) |
| create_account | 🟩 **1437** | 🟥 2978 (+1541) | 🟩 1470 (+33) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **18320** | 🟥 187632 (+169312) | 🟩 18912 (+592) |
