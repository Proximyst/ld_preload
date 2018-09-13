# ld_preload

ld_preload is a library meant for easily creating LD_PRELOAD constructors and destructors, like one would use `__attribute__((constructor))` and `__attribute((destructor))` for in CXX with GCC. This has only been tested to work with GNU/Linux.

# Usage

In order to use the library, use the macros:

```rust
#[macro_use]
extern crate ld_preload;

ld_preload_init {
    println!("Hello, Ferris!");
}

ld_preload_deinit {
    println!("Goodbye, Ferris!");
}
```

Then you need to set `lib.crate-type` to `["cdylib"]` to compile it.

To use it, you may do something along these lines: `LD_PRELOAD=./target/release/libLIBNAME.so echo "I love Ferris!"`

It may also be useful to set `profile.dev.panic` and `profile.release.panic` to `"abort"`.
