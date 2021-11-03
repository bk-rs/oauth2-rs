# Create certs

```
CAROOT=$(pwd) mkcert -install
CAROOT=$(pwd) mkcert oauth2-rs.lvh.me
```

# Installing the CA on other systems

Ref https://github.com/FiloSottile/mkcert#installing-the-ca-on-other-systems

```
CAROOT=$(pwd) mkcert -install
```
