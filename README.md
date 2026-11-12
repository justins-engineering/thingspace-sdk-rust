# Thingspace SDK Rust
A unofficial SDK for interacting with the [Verizon ThingSpace](https://thingspace.verizon.com/documentation.html) Standard APIs.

## Testing
Run every gate CI runs, from any directory:

```sh
./test.sh                # fmt, build, clippy, test, doc; one PASS or FAIL line each
./test.sh clippy test    # only the gates named
SKIP="doc" ./test.sh     # all but doc, shown as SKIP in the summary
./test.sh help           # what each gate runs
```

The gates are `fmt`, `build`, `clippy`, `test` and `doc`. Nothing in them talks to Verizon: five
failed logins lock a ThingSpace account, so `examples/native.rs` and `secrets.toml` stay out of it.
It needs the `wasm32-unknown-unknown` target and the `rustfmt` and `clippy` components.
