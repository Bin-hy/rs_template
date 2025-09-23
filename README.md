
# 如何启动？

## 启动docker依赖
1. 创建一个`.env.local`，复制 `.env.example` 
2. 运行 `docker-compose up -d` 启动依赖

## 启动服务

```powershell
cargo run -- -c conf/server.toml
```