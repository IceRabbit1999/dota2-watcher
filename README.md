# DOTA2 Watcher

[English](README_en.md)

DOTA2 Watcher是一个可以监控群友们的DOTA2战绩，以便在第一时间安慰输掉比赛之后的伤心群友，维护群友之间真挚情感的小玩具。由两部分组成：

+ Web Server：Rust + Axum实现
+ 微信bot：Wechaty实现（Typescript）
+ 为什么使用这样画蛇添足的实现方式：hands on Axum and Typescript


**注意：强烈建议使用微信小号扫码登陆！！！**

**注意：强烈建议使用微信小号扫码登陆！！！**

**注意：强烈建议使用微信小号扫码登陆！！！**

目前在各个微信bot实现方案的Issue里基本都会看到有被封禁的情况

## 快速开始

### Step1: 环境配置
由于使用了Axum和Wechaty来实现，自然需要满足这两个框架的环境，如何配置请参考他们在github上的文档：
+ [Axum](https://github.com/tokio-rs/axum)
+ [Wechaty](https://github.com/wechaty/wechaty)

### Step2：构建配置文件
DOTA2 Watcher启动时会读取`config/app.toml`作为配置文件，配置文件很简单：

```toml
# STEAM API KEY
api="xxx"

[log]
# 日志文件前缀
prefix="xxx"
# 日志文件存放路径
path="xxx"
```

### Step3：启动

TODO

## Trouble Shooting
