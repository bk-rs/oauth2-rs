# TikTok Authorization Code Grant

Ref [TikTok Display API](https://developers.tiktok.com/doc/display-api-overview/)

## Prerequisites

1. Create an App

Ref https://developers.tiktok.com/doc/getting-started-create-an-app/

```
App icon: Require 600x600

App name: oauth2-rs-demo
Category: Utilities
Description: OAuth2 test


Platform -> Configure for Web
Website URL: https://oauth2-rs.lvh.me


Products -> Login Kit
Terms of Service URL: https://oauth2-rs.lvh.me/privacy
Privacy Policy URL: https://oauth2-rs.lvh.me/privacy
Redirect domain:
  oauth2-rs.lvh.me


Products -> TikTok API -> Enable scopes
user.info.basic
video.list
```

Note: Require "Submit for review" and wait status changed to "Live in production". Otherwise "Something went wrong" with redirect_uri .
