# Binary size

`cargo build --release` on x86_64-linux-gnu, measured with `stat -c %s target/release/scaffold`.

| Change | Bytes |
| --- | ---: |
| Baseline: thin LTO, codegen-units 1, strip | 812640 |
| panic = "abort" | 757936 |
