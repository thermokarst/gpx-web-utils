# gpx-web-utils

## development

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
trunk serve
```

## deployment

```bash
trunk build --release
git switch deploy
rm index-*
cp dist/* .
git commit -am 'foo'
git push <DOKKU INSTANCE> deploy
```
