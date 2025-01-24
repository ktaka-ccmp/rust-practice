# route guide

- https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md

## server and client

server

```bash
cargo run --bin server

or

cargo build --bin server
./target/debug/server
```

client

```bash
cargo run --bin client

or

cargo build --bin client
./target/debug/client
```

## grpcurl

```bash
grpcurl -plaintext -proto ./proto/route_guide.proto -d '{"latitude":2443, "longitude":43534}' '[::1]:10000' routeguide.RouteGuide/GetFeature
```

```bash
grpcurl -plaintext -proto ./proto/route_guide.proto -d '{"lo": {"latitude": 400000000, "longitude": -750000000}, "hi": {"latitude": 420000000, "longitude": -730000000}}' '[::1]:10000' routeguide.RouteGuide/ListFeatures
```
