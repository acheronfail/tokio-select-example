For this https://github.com/tokio-rs/tokio/discussions/5684

To reproduce:

```bash
cargo build;
cat | (./target/debug/tokio-select-example; echo exited with: $?)
```

Wait for it to error, and you will observe that the program **will not exit** until a line has been read.