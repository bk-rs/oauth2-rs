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

In tab "OAuth consent screen"

```
App information:

    name: oauth2-lite-demo

App domain:

    home page: http://oauth2-lite.lvh.me

Authorized domains:

    lvh.me

Developer contact information: YOUR_GMAIL
```

In tab "Scopes", Ref https://developers.google.com/identity/protocols/oauth2/limited-input-device#allowedscopes

```
email
openid
profile
https://www.googleapis.com/auth/drive.appdata
https://www.googleapis.com/auth/drive.file
https://www.googleapis.com/auth/youtube
https://www.googleapis.com/auth/youtube.readonly
```

In tab "Test users"

```
YOUR_GMAIL
```

### Create credentials

Open https://console.cloud.google.com/apis/credentials

Click "Create credentials" => "OAuth client ID"

```
Application type: TVs and Limited Input devices

Name: oauth2-lite-device
```

Download JSON (file name is client_secret_YOUR_CLIENT_ID.json)

e.g.

```
Your Client ID: "x*72"

Your Client Secret: "x*35"
```

## Steps

### Step 1: Request device and user codes

```
curl -H 'Content-Type: application/json' -d '{"client_id": "YOUR_CLIENT_ID", "scope": "email openid profile https://www.googleapis.com/auth/youtube https://www.googleapis.com/auth/youtube.readonly https://www.googleapis.com/auth/drive.file"}' -H 'Accept: application/json' https://oauth2.googleapis.com/device/code -v
```

when success, status is 200, body is

```
{
  "device_code": "DEVICE_CODE__x*98",
  "user_code": "XXXX-XXXX",
  "expires_in": 1800,
  "interval": 5,
  "verification_url": "https://www.google.com/device"
}
```

when scope include "https://www.googleapis.com/auth/drive.appdata", status is 400, body is

```
{
  "error": "invalid_scope"
}
```

### Step 2

Open https://www.google.com/device , the `verification_url`

Input `XXXX-XXXX` , the `user_code`

### Step 3: Obtain access_token

```
curl -H 'Content-Type: application/json' -d '{"client_id": "YOUR_CLIENT_ID", "client_secret": "YOUR_CLIENT_SECRET", "device_code": "DEVICE_CODE", "grant_type": "urn:ietf:params:oauth:grant-type:device_code"}' -H 'Accept: application/json' https://oauth2.googleapis.com/token -v
```

when success, status is 200, body is

```
{
  "access_token": "x*163",
  "expires_in": 3599,
  "refresh_token": "x*103",
  "scope": "https://www.googleapis.com/auth/userinfo.profile openid https://www.googleapis.com/auth/youtube https://www.googleapis.com/auth/drive.file https://www.googleapis.com/auth/youtube.readonly https://www.googleapis.com/auth/userinfo.email",
  "token_type": "Bearer",
  "id_token": "x*1145"
}
```

when not input user_code, status is 428, body is

```
{
  "error": "authorization_pending",
  "error_description": "Precondition Required"
}
```

### Step 4: Test

```
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" https://www.googleapis.com/youtube/v3/channels?part=snippet&mine=true -v
```

```
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" https://www.googleapis.com/drive/v2/files -v
```

### Step 5: Refresh access_token

```
curl -H 'Content-Type: application/json' -d '{"client_id": "YOUR_CLIENT_ID", "client_secret": "YOUR_CLIENT_SECRET", "grant_type": "refresh_token", "refresh_token": "YOUR_REFRESH_TOKEN"}' -H 'Accept: application/json' https://oauth2.googleapis.com/token -v
```

when success, status is 200, body like `Obtain access_token`

## Note

About [Refresh token expiration](https://developers.google.com/identity/protocols/oauth2#expiration)
