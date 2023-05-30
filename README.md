# Midas
Cause everything it touches turns to gold or something, idk.
## Board Representation
### Square Mapping Considerations
#### Deduction on Files and Ranks
We use LSF-mapping (Least Significant File, i.e. iterating through ranks at the outer level, then files at the inner level).
```
square_idx = 8 * rank + file;
file = square_idx % 8 = square_idx & 7;
rank = square_idx / 8 = square_idx >> 3;
```

#### Endianness
We use little endian mapping to retain the nice relations (a < h and 0 < 7).

#### Little-Endian Rank-File Mapping
!(LERF visualization on chessboard)[lerf.jpg]

## Debugging
## Search
## Evaluation
## UCI
