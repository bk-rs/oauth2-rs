# Microsoft Device Authorization Grant

Ref [OAuth 2.0 device authorization grant flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code)

## Prerequisites

1. Select / Register the app in the portal

Same as [Authorization_Code_Grant.md](./Authorization_Code_Grant.md)

2. Enable "No keyboard (Device Code Flow)"

In tab "Authentication"

```
Allow public client flows
    Enable the following mobile and desktop flows:
        No keyboard (Device Code Flow) Learn more
```
