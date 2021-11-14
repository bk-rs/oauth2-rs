# Microsoft Authorization Code Grant

Ref [Microsoft identity platform and OAuth 2.0 authorization code flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)

Ref [Build Ruby on Rails apps with Microsoft Graph](https://docs.microsoft.com/en-us/graph/tutorials/ruby)

## Prerequisites

1. Register the app in the portal

"Azure Active Directory" > "App registrations"

Click "New registration"

```
Name: oauth2-rs-demo

Supported account types: "Accounts in any organizational directory (Any Azure AD directory - Multitenant) and personal Microsoft accounts (e.g. Skype, Xbox)"

Redirect URI:
    Web  https://oauth2-rs.lvh.me/auth/microsoft/callback
```

```
Application (client) ID: UUID (This is client_id)
Directory (tenant) ID: UUID
```

2. Add a certificate or secret

In tab "Certificates & secrets"

Click "New client secret"

Description: web-app

```
Value: x*37 (This is client_secret)
Secret ID: UUID
```
