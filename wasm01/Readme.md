# Readme

```text
cargo add wasm_bindgen
wasm-pack build --target web
docker run --rm -p 8000:8000 -v "$(pwd)":/usr/src/app -w /usr/src/app python:3 python -m http.server 8000
```
