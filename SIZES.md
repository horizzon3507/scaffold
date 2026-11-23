# Binary size

`cargo build --release` on x86_64-linux-gnu, measured with `stat -c %s target/release/scaffold`.

| Change | Bytes |
| --- | ---: |
| Baseline: thin LTO, codegen-units 1, strip | 812640 |
| panic = "abort" | 757936 |
| opt-level = "z" | 616096 |
| lto = "fat" | 616112 |
| opt-level = "s" measured 645632, reverted to z | 616096 |
