# GitHub Device Authorization Grant

Ref [Authorizing OAuth Apps - Device flow](https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#device-flow)

## Prerequisites

### Create OAuth App

Open https://github.com/settings/developers

Click "New OAuth App"

```
Application name: oauth2-rs-demo

Homepage URL: http://oauth2-rs.lvh.me

Authorization callback URL: http://oauth2-rs.lvh.me/auth/github/callback
```

Click "Generate a new client secret" in app detail page

```
Your Client ID: "x*20"

Your Client Secret: "x*40"
```

Note: Maybe client_secret is not required

## Steps

### Step 1

Ref https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#step-1-app-requests-the-device-and-user-verification-codes-from-github

```
curl -H 'Content-Type: application/json' -d '{"client_id": "YOUR_CLIENT_ID", "scope": "user:email public_repo"}' -H 'Accept: application/json' https://github.com/login/device/code -v
```

when success, status is 200, body is

```
{"device_code":"DEVICE_CODE__x*40","user_code":"XXXX-XXXX","verification_uri":"https://github.com/login/device","expires_in":899,"interval":5}
```

### Step 2

Ref https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#step-2-prompt-the-user-to-enter-the-user-code-in-a-browser

Open https://github.com/login/device , the `verification_url`

Input `XXXX-XXXX` , the `user_code`

### Step 3

```
curl -H 'Content-Type: application/json' -d '{"client_id": "YOUR_CLIENT_ID", "device_code": "DEVICE_CODE", "grant_type": "urn:ietf:params:oauth:grant-type:device_code"}' -H 'Accept: application/json' https://github.com/login/oauth/access_token -v
```

when success, status is 200, body is

```
{"access_token":"x*40","token_type":"bearer","scope":"public_repo,user:email"}
```

when not input user_code, status is 400, body is

```
{"error":"authorization_pending","error_description":"The authorization request is still pending.","error_uri":"https://docs.github.com/developers/apps/authorizing-oauth-apps#error-codes-for-the-device-flow"}
```

### Step 4: Test

```
curl -H "Authorization: token YOUR_ACCESS_TOKEN" https://api.github.com/user -v
```
