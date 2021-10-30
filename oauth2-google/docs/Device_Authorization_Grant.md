# Google Device Authorization Grant

Ref [OAuth 2.0 for TV and Limited-Input Device Applications](https://developers.google.com/identity/protocols/oauth2/limited-input-device)

## Prerequisites

Project name: "oauth2-lite-demo"

### Enable APIs

Open https://console.developers.google.com/apis/library

Enable "Google Drive API"

Enable "YouTube Data API", e.g. "YouTube Data API v3"

### Create OAuth consent screen

Open https://console.cloud.google.com/apis/credentials/consent

Select "External" (User Type) then click "CREATE"

On tab "OAuth consent screen"

```
App information:

    name: oauth2-lite-demo

App domain:

    home page: http://oauth2-lite.lvh.me

Authorized domains:

    lvh.me

Developer contact information: YOUR_GMAIL
```

On tab "Scopes", Ref https://developers.google.com/identity/protocols/oauth2/limited-input-device#allowedscopes

```
email
openid
profile
https://www.googleapis.com/auth/drive.appdata
https://www.googleapis.com/auth/drive.file
https://www.googleapis.com/auth/youtube
https://www.googleapis.com/auth/youtube.readonly
```

On tab "Test users"

```
YOUR_GMAIL
```

### Create credentials

Click "Create credentials" => "OAuth client ID"

```
Application type: TVs and Limited Input devices

Name: oauth2-lite-device
```

Download JSON (file name is client_secret_YOUR_CLIENT_ID.json)

e.g.

```
Your Client ID: x*72

Your Client Secret: x*35
```
