# Binary size

`cargo build --release` on x86_64-linux-gnu, measured with `stat -c %s target/release/scaffold`.

| Change | Bytes |
| --- | ---: |
| Baseline: thin LTO, codegen-units 1, strip | 812640 |
| panic = "abort" | 757936 |
| opt-level = "z" | 616096 |
| lto = "fat" | 616112 |
| opt-level = "s" measured 645632, reverted to z | 616096 |
| Hand-rolled parser, clap removed | 421312 |
| anyhow removed: zero dependencies | 401328 |
| Shell completions added | 403896 |
| Colors and --no-color added | 407032 |
| Config file support added | 412456 |
| --json output added | 419864 |
| latex, zig, deno, flask presets added | 424104 |
| --quiet flag added | 424360 |
| v0.2.0 release | 424360 |
| docker, astro, make, terraform presets added | 428024 |
| describe --tree added | 435720 |
| Interactive picker added | 447752 |
| scaffold init added | 450328 |
| scaffold doctor added | 452392 |
| describe --show added | 453080 |
| SIGPIPE fix | 453112 |
