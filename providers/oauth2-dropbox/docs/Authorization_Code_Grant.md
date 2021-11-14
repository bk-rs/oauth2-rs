# Dropbox Authorization Code Grant

Ref [OAuth Guide](https://developers.dropbox.com/oauth-guide)

## Prerequisites

1. Create app

https://www.dropbox.com/developers/apps

```
Choose an API: Scoped access
Choose the type of access you need: App folder
Name your app: oauth2-rs-demo
```

2. App Settings

```
Client ID: "x*15" (App key)

Client Secret: "x*15" (App secret)
```

```
OAuth 2
    Redirect URIs: https://oauth2-rs.lvh.me/auth/dropbox/callback
```

3. App Permissions

```
account_info.read

sharing.read
```
