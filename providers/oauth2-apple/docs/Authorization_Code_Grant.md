# Apple Authorization Code Grant

Ref [Incorporating Sign in with Apple into Other Platforms](https://developer.apple.com/documentation/sign_in_with_apple/sign_in_with_apple_js/incorporating_sign_in_with_apple_into_other_platforms)

Ref [Generate and Validate Tokens](https://developer.apple.com/documentation/sign_in_with_apple/generate_and_validate_tokens)

## Prerequisites

Ref [What the Heck is Sign In with Apple?](https://developer.okta.com/blog/2019/06/04/what-the-heck-is-sign-in-with-apple)

Require Apple Developer Program

1. Register an App ID

```
Bundle ID: com.xxx.oauth2-app

Enable "Sign In with Apple"
```

2. Register a Services ID

```
Identifier: com.xxx.oauth2-services
```

3. Edit your Services ID Configuration

Enable "Sign In with Apple", then Configure, select "Primary App ID" to "******.com.xxx.oauth2-app"

```
Domains and Subdomains:
    oauth2-rs.lvh.me

Return URLs
    https://oauth2-rs.lvh.me/auth/apple/callback
```

4. Register a New Key

```
Key Name: xxx oauth2

Enable "Sign In with Apple", then Configure, select "Primary App ID" to "******.com.xxx.oauth2-app"
```

Download filename is "AuthKey_{Key ID}.p8"

5. Creating the Client Secret

"Team ID" in https://developer.apple.com/account/#!/membership

"client_id" is The Identifier for Services ID, e.g. "com.xxx.oauth2-services"

Ref https://github.com/bk-rs/apple-rs/blob/main/apple-siwa-client-secret/cli/src/bin/apple_siwa_client_secret_gen.rs

JWT HEADER

```
{
    "alg": "ES256",
    "kid": "{Key ID}"
}
```

JWT PAYLOAD

```
{
    "iss": "{Team ID}",
    "iat": {Current Unix Timestamp},
    "exp": {Current Unix Timestamp + 15777000},
    "aud": "https://appleid.apple.com",
    "sub": "{client_id}"
}
```

Note: exp max 6 months
