# Readme

- https://youtu.be/kerKXChDmsE
- https://github.com/dreamsofcode-io/grpcalculator-rs

## howto

```bash
rustup  update 
cargo new calculator-grpc
cd calculator-grpc/

cargo add tonic
cargo add tokio --features=full
cargo add prost
cargo add --build tonic-build

sudo aptitude install protobuf-compiler
```

```bash
Download grcurl from https://github.com/fullstorydev/grpcurl/releases

sudo apt install ./Downloads/grpcurl_1.9.2_linux_amd64.deb 
```

```bash
cargo run 

grpcurl -plaintext -proto ./proto/calculator.proto -d '{"a":2,"b":3}' '[::1]:50051' calculator.Calculator.Add
{
  "result": "5"
}

grpcurl -plaintext -proto ./proto/calculator.proto -d '{"a":2,"b":8}' '[::1]:50051' calculator.Calculator.Add
{
  "result": "10"
}
```

## note

rust-analyzer complains that:
> OUT_DIR not set, enable "build scripts" to fixrust-analyzermacro-error

Claude:
This is a common issue with rust-analyzer and tonic/protobuf projects. The error occurs because rust-analyzer doesn't know where to find the generated protobuf code, which is created by the build script in the `OUT_DIR`.

To fix this, you need to configure rust-analyzer to recognize build scripts.

1. If you're using VS Code, you can add this to your `.vscode/settings.json`:
```json
{
    "rust-analyzer.cargo.buildScripts.enable": true
}
```

After adding either of these configurations and reloading your editor, rust-analyzer should properly recognize the generated protobuf code and the error should go away.

## grpc-reflection

```bash
cargo add tonic-reflection
```

```bash
$ grpcurl -plaintext -d '{"a":2,"b":8}' '[::1]:50051' calculator.Calculator.Add
{
  "result": "10"
}

$ grpcurl -plaintext '[::1]:50051' list
calculator.Calculator
grpc.reflection.v1.ServerReflection
```

## grpuc

- https://github.com/fullstorydev/grpcui/releases

```bash
wget https://github.com/fullstorydev/grpcui/releases/download/v1.4.2/grpcui_1.4.2_linux_x86_64.tar.gz
tar xvf grpcui_1.4.2_linux_x86_64.tar.gz grpcui

$ ./grpcui -plaintext '[::1]:50051' 
gRPC Web UI available at http://127.0.0.1:42463/
Opening in existing browser session.
```

## RequestCounter

```bash
$ grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
{
  "count": "1"
}

$ grpcurl -plaintext -d '{"a":8,"b":3}' '[::1]:50051' calculator.Calculator.Add
{
  "result": "11"
}

$ grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
{
  "count": "2"
}

$ grpcurl -plaintext -d '{"a":8,"b":3}' '[::1]:50051' calculator.Calculator.Divide
{
  "result": "2"
}

$ grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
{
  "count": "3"
}
```

## Intercepor

```bash
$ grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
ERROR:
  Code: Unauthenticated
  Message: missing token

$ grpcurl -H "Authorization: Bearer secret-token" -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
{
  "count": "0"
}
```

一文字でも異なるとだめ

```bash
$ grpcurl -H "Authorization: Bearer secret-toke" -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
ERROR:
  Code: Unauthenticated
  Message: missing token
```

## gRPC Web

-https://github.com/dreamsofcode-io/grpcalculator-web/tree/main
