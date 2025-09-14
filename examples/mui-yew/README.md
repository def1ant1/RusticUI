# MUI Yew Example

This example demonstrates how to combine the `mui-system` crate with the
[Yew](https://yew.rs) framework.  It is configured for production builds so the
output can be served from a static host or CDN.

## Development

```bash
trunk serve --open
```

## Production

```bash
trunk build --release
# Deploy the `dist/` directory behind a CDN such as Vercel or Cloudflare
# Workers for globally distributed hosting.
```

The release build enables `wasm-opt` by default which keeps the WebAssembly
binary small and fast to download.
