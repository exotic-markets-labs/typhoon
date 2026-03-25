## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `vanilla`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **11** | 🟥 226 (+215) | 🟩 13 (+2) | 🟩 **11** |
| log | 🟩 117 (+1) | 🟥 330 (+214) | 🟩 117 (+1) | 🟩 **116** |
| create_account | 🟩 1575 (+131) | 🟥 3708 (+2264) | 🟩 **1444** | 🟩 1541 (+97) |
| transfer | 🟩 **1290** | 🟨 2405 (+1115) | 🟩 1303 (+13) | 🟩 1316 (+26) |
| unchecked_accounts | 🟩 **99** | 🟥 1745 (+1646) | 🟩 101 (+2) | 🟩 104 (+5) |
| accounts | 🟩 323 (+31) | 🟥 1751 (+1459) | 🟩 **292** | 🟩 356 (+64) |

### Binary Size

|                     | `vanilla`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 **18688** | 🟥 137456 (+118768) | 🟩 19352 (+664) | 🟥 114608 (+95920) |
