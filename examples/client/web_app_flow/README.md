## Dev

```
cp config/clients.toml.example config/clients.toml
CAROOT=$(pwd)/mkcert mkcert -install


RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth2_client_web_app_flow_example --bin web_app_flow_axum
# or
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth2_client_web_app_flow_example --bin web_app_flow_warp


sudo socat tcp-listen:80,reuseaddr,fork tcp:127.0.0.1:8080
sudo socat tcp-listen:443,reuseaddr,fork tcp:127.0.0.1:8443


xdg-open http://oauth2-rs.lvh.me/auth/github
xdg-open https://oauth2-rs.lvh.me/auth/google
xdg-open https://oauth2-rs.lvh.me/auth/twitch
xdg-open http://oauth2-rs.lvh.me/auth/mastodon-social
xdg-open https://oauth2-rs.lvh.me/auth/apple
xdg-open https://oauth2-rs.lvh.me/auth/instagram
xdg-open https://oauth2-rs.lvh.me/auth/facebook
xdg-open https://oauth2-rs.lvh.me/auth/amazon
xdg-open http://oauth2-rs.lvh.me/auth/gitlab
xdg-open http://oauth2-rs.lvh.me/auth/bitbucket
xdg-open http://oauth2-rs.lvh.me/auth/digitalocean
xdg-open https://oauth2-rs.lvh.me/auth/dropbox
xdg-open http://oauth2-rs.lvh.me/auth/linkedin
xdg-open https://oauth2-rs.lvh.me/auth/microsoft
xdg-open https://oauth2-rs.lvh.me/auth/yahoo
xdg-open https://oauth2-rs.lvh.me/auth/okta
xdg-open https://oauth2-rs.lvh.me/auth/pinterest
xdg-open https://oauth2-rs.lvh.me/auth/baidu
xdg-open https://oauth2-rs.lvh.me/auth/twitter
xdg-open https://oauth2-rs.lvh.me/auth/tiktok
xdg-open http://oauth2-rs.lvh.me/auth/zoho
xdg-open https://oauth2-rs.lvh.me/auth/linode
```
