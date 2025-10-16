
# 如何启动？

## 启动docker依赖
1. 创建一个`.env.local`，复制 `.env.example`
2. 运行 `docker-compose up -d` 启动依赖

## 启动服务

```powershell
cargo run -- -c conf/server.toml
```

## 数据库表约定规范
> 以 `_`开头的是 权限管理所用的表


# 开发

## 安装相关依赖

### sea-orm-cli
```powershell
cargo install sea-orm-cli
```

# 数据库迁移
> 每次修改数据库表结构后，都需要运行以下命令迁移数据库
```powershell
sea-orm-cli migrate up
```

# 测试
> 运行以下命令测试服务
```powershell
cargo test
```
