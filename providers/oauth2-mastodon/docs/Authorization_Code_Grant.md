# Mastodon Authorization Code Grant

Ref [OAuth](https://docs.joinmastodon.org/spec/oauth/)

## Prerequisites

### Create OAuth App

Open https://mastodon.social/settings/applications

Click "NEW APPLICATION"

```
Application name: oauth2-rs-demo

Redirect URI: http://oauth2-rs.lvh.me/auth/mastodon-social/callback

Scopes: read write follow
```

```
Your Client key: "x*43"

Your Client secret: "x*43"

Your access token: "x*43" # This is "App token" and "User token"
```

### Create OAuth App for Mobile & Desktop Apps

```
Application name: oauth2-rs-desktop-demo

Redirect URI: urn:ietf:wg:oauth:2.0:oob

Scopes: read write follow
```
