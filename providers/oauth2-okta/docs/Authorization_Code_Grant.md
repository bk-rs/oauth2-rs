# Okta Authorization Code Grant

Ref [Create an OAuth 2.0 app in Okta](https://developer.okta.com/docs/guides/implement-oauth-for-okta/-/create-oauth-app/)

## Prerequisites

1. Create App Integration

```
Sign-in method: OIDC - OpenID Connect

Application type: Web Application
```

```
App integration name: oauth2-rs-web-demo

Grant type: Authorization Code

Sign-in redirect URIs: https://oauth2-rs.lvh.me/auth/okta/callback

Controlled access: Allow everyone in your organization to access
```

```
Client ID: "x*20"

Client secret: "x*40"

Okta domain: "dev-xxxxxxxx.okta.com"
```
