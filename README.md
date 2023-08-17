# Release build
```shell
cargo build --target armv7-unknown-linux-musleabihf --release
```

# Generate SSL certificate 

```shell
openssl genrsa -traditional -out key.pem 4096
openssl req -new -x509 -key key.pem -out cert.pem -config openssl.cnf -days 99999

```
