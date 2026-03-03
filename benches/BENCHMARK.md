## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `vanilla`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **11** | 🟥 229 (+218) | 🟩 13 (+2) | 🟩 **11** |
| log | 🟩 117 (+1) | 🟥 333 (+217) | 🟩 117 (+1) | 🟩 **116** |
| create_account | 🟩 1575 (+131) | 🟥 3759 (+2315) | 🟩 **1444** | 🟩 1552 (+108) |
| transfer | 🟩 **1290** | 🟨 2444 (+1154) | 🟩 1303 (+13) | 🟩 1316 (+26) |
| unchecked_accounts | 🟩 **99** | 🟥 1766 (+1667) | 🟩 101 (+2) | 🟩 104 (+5) |
| accounts | 🟩 323 (+31) | 🟥 1783 (+1491) | 🟩 **292** | 🟩 356 (+64) |

### Binary Size

|                     | `vanilla`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 **18688** | 🟥 208520 (+189832) | 🟩 19360 (+672) | 🟥 114632 (+95944) |
