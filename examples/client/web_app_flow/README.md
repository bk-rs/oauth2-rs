## Dev

```
cp config/clients.toml.example config/clients.toml
CAROOT=$(pwd)/mkcert mkcert -install


RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth2_client_web_app_flow_example --bin warp
# or
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth2_client_web_app_flow_example --bin poem


sudo socat tcp-listen:80,reuseaddr,fork tcp:127.0.0.1:8080
sudo socat tcp-listen:443,reuseaddr,fork tcp:127.0.0.1:8443

xdg-open http://oauth2-rs.lvh.me/auth/github
xdg-open https://oauth2-rs.lvh.me/auth/google
```
