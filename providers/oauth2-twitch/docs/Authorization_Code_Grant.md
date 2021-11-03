# Twitch Authorization Code Grant

Ref [OAuth authorization code flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#oauth-authorization-code-flow)

## Prerequisites

### Create OAuth App

Note: "You must have two factor enabled to manage applications"

Open https://dev.twitch.tv/console/apps

Click "Register Your Application"

```
Name: oauth2-rs-demo

# Note: Must use HTTPS protocol
OAuth Redirect URLs: https://oauth2-rs.lvh.me/auth/twitch/callback

Category: Website Integration
```

```
Your Client ID: "x*30"

Your Client Secret: "x*30"
```
