# A Minimal Kernel written in Rust 

Followed by the [OS Guide](https://os.phil-opp.com/)

## How to use 
Use the nightly compiler and add the following configuration to `~/.cargo/config.toml`: 

```{toml}
[unstable]
build-std = ["core", "compiler_builtins", "alloc"]
build-std-features = ["compiler-builtins-mem"]
[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

Build: `make build` 
Run: `make run`


To run the tests use `make test` and to test heap allocations run `make test_alloc`. 