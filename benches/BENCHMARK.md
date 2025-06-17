## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **11** | 🟥 180 (+169) | 🟩 13 (+2) |
| log | 🟩 **117** | 🟥 284 (+167) | 🟩 118 (+1) |
| transfer | 🟩 **1605** | 🟥 4444 (+2839) | 🟩 1718 (+113) |
| create_account | 🟩 **1437** | 🟥 2978 (+1541) | 🟩 1471 (+34) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **18320** | 🟥 187632 (+169312) | 🟩 19768 (+1448) |
