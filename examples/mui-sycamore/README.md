# MUI Sycamore Example

Simple demo combining `rustic_ui_system` with the
[Sycamore](https://sycamore-rs.netlify.app) reactive framework.

## Usage

### Client side
```bash
trunk serve --open
```
Any static file server can host the resulting `dist/` directory.

### Server side rendering
```bash
cargo run --manifest-path examples/rustic_ui_sycamore/Cargo.toml --features ssr
```
The printed HTML can be embedded in a server response and hydrated on the
client using the CSR build above.
