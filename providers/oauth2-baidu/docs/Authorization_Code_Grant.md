# Baidu Authorization Code Grant

Ref [授权码模式授权](https://pan.baidu.com/union/doc/al0rwqzzl)

Ref [百度OAuth](https://developer.baidu.com/wiki/index.php?title=docs/oauth)

## Prerequisites

1. Create an App

Ref [创建应用](https://pan.baidu.com/union/doc/fl0hhnulu)

Open https://pan.baidu.com/union/console/applist

Click "创建应用"

```
应用类别: 软件

应用名称: oauth2-rs-demo
```

Ref [授权回调地址](https://pan.baidu.com/union/doc/Vl19c4jnx)

Open https://pan.baidu.com/union/console/app/YOUR_APP_ID

Click "安全设置"

```
OAuth授权回调页: https://oauth2-rs.lvh.me/auth/baidu/callback
```
