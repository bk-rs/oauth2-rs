# Amazon Authorization Code Grant

Ref [Login with Amazon for Websites Overview](https://developer.amazon.com/docs/login-with-amazon/web-docs.html)

Ref [Authorization Code Grant](https://developer.amazon.com/docs/login-with-amazon/authorization-code-grant.html)

## Prerequisites

1. Create a New Security Profile

```
Security Profile Name: oauth2-rs-web-demo
Security Profile Description: oauth2-rs-web-demo
Consent Privacy Notice URL: YOUR_URL e.g. https://policies.google.com/privacy
```

```
Client ID: amzn1.application-oa2-client.x*32

Client Secret: x*64
```

2. Add your Website to your Security Profile

Tab "Web Settings"

```
Allowed Origins: https://oauth2-rs.lvh.me
Allowed Return URLs: https://oauth2-rs.lvh.me/auth/amazon/callback
```
