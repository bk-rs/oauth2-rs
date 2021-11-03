# Google Authorization Code Grant

Ref [Using OAuth 2.0 for Web Server Applications](https://developers.google.com/identity/protocols/oauth2/web-server)

## Prerequisites

Project name: "oauth2-rs-demo"

### Enable APIs

Open https://console.developers.google.com/apis/library

TODO

### Create credentials

Open https://console.cloud.google.com/apis/credentials

Click "Create credentials" => "OAuth client ID"

```
Application type: Web application

Name: oauth2-rs-web-app

redirect URIs: https://oauth2-rs.lvh.me/auth/google/callback
```

Download JSON (file name is client_secret_YOUR_CLIENT_ID.json)

e.g.

```
Your Client ID: "x*72"

Your Client Secret: "x*35"
```
