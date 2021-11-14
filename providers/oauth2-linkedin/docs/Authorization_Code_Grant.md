# LinkedIn Authorization Code Grant

Ref [Authorizing OAuth Apps - Authorization Code Flow](https://docs.microsoft.com/en-us/linkedin/shared/authentication/authorization-code-flow)

## Prerequisites

1. Create app

https://www.linkedin.com/developers/apps

```
App name: oauth2-rs-demo

LinkedIn Page: 

App logo: 
```

2. App Auth

```
Client ID: "x*14"

Client Secret: "x*15"
```

```
OAuth 2.0 settings
    Authorized redirect URLs for your app: http://oauth2-rs.lvh.me/auth/linkedin/callback
```

3. App Products

Select "Sign In with LinkedIn"

