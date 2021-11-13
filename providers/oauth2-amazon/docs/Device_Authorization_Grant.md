# Amazon Device Authorization Grant

Ref [LWA for TVs and Other Devices](https://developer.amazon.com/docs/login-with-amazon/other-platforms-cbl-docs.html)

## Prerequisites

1. Create a New Security Profile

```
Security Profile Name: oauth2-rs-device-demo
Security Profile Description: oauth2-rs-device-demo
Consent Privacy Notice URL: YOUR_URL e.g. https://policies.google.com/privacy
```

```
Client ID (Don't use): amzn1.application-oa2-client.x*32

Client Secret: x*64
```

2. Add your CBL Application to your Security Profile

Tab "TVs and Other Devices Settings"

```
Device Name: Test
```

```
Client Id: "amzn1.application-oa2-client.x*32"
```
