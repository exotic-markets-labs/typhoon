## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark          | `pinocchio`   | `anchor`        | `typhoon`     | `star-frame`  |
| ------------------ | ------------- | --------------- | ------------- | ------------- |
| ping               | 🟩 **11**     | 🟥 256 (+245)   | 🟩 13 (+2)    | 🟩 13 (+2)    |
| log                | 🟩 **117**    | 🟥 360 (+243)   | 🟩 **117**    | 🟩 **117**    |
| create_account     | 🟩 1583 (+27) | 🟥 4294 (+2738) | 🟩 1612 (+56) | 🟩 **1556**   |
| transfer           | 🟩 **1289**   | 🟥 2867 (+1578) | 🟩 1345 (+56) | 🟩 1324 (+35) |
| unchecked_accounts | 🟩 **96**     | 🟥 2053 (+1957) | 🟩 99 (+3)    | 🟩 103 (+7)   |
| accounts           | 🟩 **363**    | 🟥 1991 (+1628) | 🟩 402 (+39)  | 🟩 416 (+53)  |

### Binary Size

|                     | `pinocchio`  | `anchor`            | `typhoon`        | `star-frame`        |
| ------------------- | ------------ | ------------------- | ---------------- | ------------------- |
| Binary size (bytes) | 🟩 **18656** | 🟥 201472 (+182816) | 🟩 21608 (+2952) | 🟥 137408 (+118752) |
