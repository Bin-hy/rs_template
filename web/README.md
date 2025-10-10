
# 如何搭建一个基本环境

## 配置tailwindcss
查看文档：[tailwindcss 3.0 配置](https://www.tailwindcss.cn/docs/guides/vite#vue)

安装开发依赖
```powershell
npm install -D tailwindcss@3 postcss autoprefixer
```

初始化tailwindcss 配置文件
```powershell
npx tailwindcss init -p
```
⚠ 注意：
- 安装tailwindcss 3.0 版本要写 *`tailwindcss@3`*
- 初始化tailwindcss 配置文件 报错就直接手动复制我的配置文件[postcss.config.js](./postcss.config.js),[tailwind.config.js](./tailwind.config.js)

附：
- [tailwindcss 3.0 配置文件](https://www.tailwindcss.cn/docs/configuration)
-
