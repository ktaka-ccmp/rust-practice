# axum google oauth2 example

もともとdiscord用の[axum/examples/oauth](https://github.com/tokio-rs/axum/blob/main/examples/oauth/src/main.rs)を改造

```text
ngrok http 3000
```

ngrokのURLをORIGINに設定

```text
export ORIGIN="https://xxxxx.ngrok-free.app"
export CLIENT_ID=$client_id
export CLIENT_SECRET=$client_secret

cargo watch -x run
```
