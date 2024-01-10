# Merlin Challenge Labels

Macro to generate fixed `&'static [u8]` labels for a merlin transcript protocol upto `STAT_PARAM` at compile time.

```
cargo check
cargo run                               # pass the static slices at runtime
cargo expand --bin merlin-challenge     # see macro expansion
```
