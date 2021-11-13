# Bitbucket Authorization Code Grant

Ref [OAuth 2.0](https://developer.atlassian.com/cloud/bitbucket/oauth-2/)

Ref [Use OAuth on Bitbucket Cloud](https://support.atlassian.com/bitbucket-cloud/docs/use-oauth-on-bitbucket-cloud/)

## Prerequisites

1. Add consumer

```
Name: oauth2-rs-demo
Callback URL: http://oauth2-rs.lvh.me/auth/bitbucket/callback
Permissions:
    Account
        Email
        Read
    Repositories
        Read
```

```
Client ID: "x*18"

Client Secret: "x*32"
```
