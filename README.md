```
$ cargo run --release -- --input data/features.ndjson --dict-size 50000000 --block-size 5000000 --level 19
uncompressed: Summary { count: 206560, total: 159217324 }
naive: 2.20 Summary { count: 206560, total: 72343587 }
block: 12.88 Summary { count: 32, total: 12358561 }
dict: 3.62 Summary { count: 206560, total: 43924888 }
```
