# Instagram Authorization Code Grant

Ref [Instagram Basic Display API - Get Started](https://developers.facebook.com/docs/instagram-basic-display-api/getting-started)

## Prerequisites

1. Create a Facebook App

Audience

```
Which option best suits the audience you’re building this app for?

Manage integrations for your business
```

Type

```
Consumer
```

Details

```
Display name: oauth2-rs-i-g-demo
```

2. Configure App, Settings > Basic

Privacy Policy URL: YOUR_URL e.g. https://policies.google.com/privacy

User Data Deletion -> Data Deletion Callback URL: https://oauth2-rs.lvh.me/fb/data_deletion_callback

Add Platform

```
Platform: Website

Site URL: https://oauth2-rs.lvh.me
```

3. Add and configure "Instagram Basic Display" Product

Create New App

```
Valid OAuth Redirect URIs: https://oauth2-rs.lvh.me/auth/instagram/callback

Deauthorize Callback URL: https://oauth2-rs.lvh.me/fb/ig_deauthorize_callback

Data Deletion Request URL: https://oauth2-rs.lvh.me/fb/ig_data_deletion_callback
```

```
Your Client ID: (Instagram App ID)

Your Client Secret: (Instagram App Secret)
```

4. Add an Instagram Test User

