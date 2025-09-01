## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 **12** | 🟥 272 (+260) | 🟩 **12** | 🟥 197 (+185) |
| log | 🟩 **117** | 🟥 376 (+259) | 🟩 **117** | 🟥 301 (+184) |
| create_account | 🟩 **1619** | 🟥 4429 (+2810) | 🟩 1662 (+43) | 🟨 2726 (+1107) |
| transfer | 🟩 **1290** | 🟥 2956 (+1666) | 🟩 1349 (+59) | 🟨 2319 (+1029) |
| unchecked_accounts | 🟩 **100** | 🟥 2064 (+1964) | 🟩 101 (+1) | 🟥 538 (+438) |
| accounts | 🟩 461 (+12) | 🟥 2123 (+1674) | 🟩 **449** | 🟥 1239 (+790) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 **18520** | 🟥 206544 (+188024) | 🟩 21888 (+3368) | 🟥 169280 (+150760) |
