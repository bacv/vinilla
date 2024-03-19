# Examples

To build `examples/vinilla-c` first build the `vinilla-lib` crate:

```bash
cargo build -p vinilla-lib --release
```

Then build an example:

```bash
gcc examples/vinilla-c/main.c -lvinilla -Ltarget/release -Itarget/ -o target/vinilla-c
```
