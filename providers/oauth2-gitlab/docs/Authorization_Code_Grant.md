# GitLab Authorization Code Grant

Ref [Configure GitLab as an OAuth 2.0 authentication identity provider](https://docs.gitlab.com/ee/integration/oauth_provider.html)

## Prerequisites

1. Create OAuth App

```
Name: oauth2-rs-demo
Redirect URI: http://oauth2-rs.lvh.me/auth/gitlab/callback
Scopes: read_user openid profile email
```

```
Client ID: "x*64"

Client Secret: "x*64"
```
