# Liwin - Rust + Dioxus + TailwindCSS 项目模板

一个基于 Rust + Dioxus + TailwindCSS 的现代化 Web 应用模板，具有模块化的目录结构和完整的开发工具链。

## 🚀 技术栈

- **Rust** - 系统编程语言
- **Dioxus** - 用于构建用户界面的 Rust 框架
- **TailwindCSS** - 实用优先的 CSS 框架
- **WebAssembly** - 高性能的 Web 技术
- **wasm-pack** - Rust 到 WebAssembly 的构建工具

## 📁 项目结构

```
Liwin/
├── src/
│   ├── components/     # 可复用组件
│   ├── pages/         # 页面组件
│   ├── router/        # 路由配置
│   └── lib.rs         # 主入口文件
├── public/            # 静态资源
├── styles/            # 样式文件
├── .github/           # GitHub Actions 配置
└── target/            # 构建输出
```

## 🛠️ 本地开发

### 环境要求

- **Rust**: 1.70+ (推荐使用 rustup 安装)
- **Node.js**: 18+ (推荐使用 nvm 安装)
- **wasm-pack**: 0.12+ (通过 cargo 安装)

### 安装步骤

1. **安装 Rust**
   ```bash
   # 下载并运行 rustup-init.exe
   # 或访问 https://rustup.rs/
   ```

2. **安装 wasm-pack**
   ```bash
   cargo install wasm-pack
   ```

3. **安装 Node.js 依赖**
   ```bash
   npm install
   ```

4. **构建项目**
   ```bash
   # Windows
   npm run build
   
   # 或手动执行
   npx tailwindcss -i ./styles/input.css -o ./public/styles/output.css
   wasm-pack build --target web --out-dir public/pkg
   ```

5. **启动开发服务器**
   ```bash
   npm run dev
   ```

6. **访问应用**
   - 打开浏览器访问: http://localhost:3000

## 🌐 GitHub Pages 部署

### 自动部署

项目已配置 GitHub Actions 自动部署到 GitHub Pages：

1. **推送代码到主分支**
   ```bash
   git add .
   git commit -m "Update project"
   git push origin main
   ```

2. **GitHub Actions 自动构建**
   - 工作流文件: `.github/workflows/deploy.yml`
   - 自动触发: 推送到 `main` 或 `master` 分支
   - 构建步骤:
     - 安装 Node.js 和 Rust 环境
     - 构建 TailwindCSS (生产模式，压缩)
     - 构建 WebAssembly (release 模式)
     - 部署到 GitHub Pages

3. **配置 GitHub Pages**
   - 进入仓库 Settings → Pages
   - Source: 选择 "GitHub Actions"
   - 确保仓库有 `pages: write` 权限

### 手动部署

如果需要手动部署：

1. **构建生产版本**
   ```bash
   npm run build:prod
   ```

2. **部署到 GitHub Pages**
   ```bash
   # 使用 gh-pages 包
   npx gh-pages -d public
   ```

### 自定义域名

1. **添加 CNAME 文件**
   ```bash
   echo "your-domain.com" > public/CNAME
   ```

2. **配置 DNS**
   - 添加 CNAME 记录指向 `username.github.io`
   - 或在 GitHub Pages 设置中配置自定义域名

## 📦 构建脚本

### 开发构建
```bash
npm run build
```

### 生产构建
```bash
npm run build:prod
```

### 清理构建
```bash
npm run clean
```

## 🔧 配置说明

### TOML 配置管理

项目支持使用 TOML 文件进行配置管理，提供了类型安全的配置结构：

#### 配置文件
- **示例配置**: `config.example.toml` - 配置模板文件
- **实际配置**: `config.toml` - 应用使用的配置文件
- **配置模块**: `src/config.rs` - 配置结构体定义和加载逻辑

#### 配置结构
```toml
[app]
name = "Liwin Demo"
version = "1.0.0"
description = "Dioxus + TailwindCSS 配置管理演示应用"

[server]
host = "127.0.0.1"
port = 3000
debug = true

[features]
auth = true
api = true
websocket = false
caching = true

[ui]
theme = "light"
language = "zh-CN"
timezone = "Asia/Shanghai"
```

#### 使用方法
1. **复制配置文件**
   ```bash
   cp config.example.toml config.toml
   ```

2. **修改配置**
   - 编辑 `config.toml` 文件
   - 所有配置项都有类型安全的结构体定义
   - 支持嵌套配置和复杂数据类型

3. **在代码中使用**
   ```rust
   use crate::config::AppConfig;
   
   // 加载配置
   let config = AppConfig::load()?;
   
   // 使用配置
   println!("应用名称: {}", config.app.name);
   println!("服务器端口: {}", config.server.port);
   ```

4. **配置演示页面**
   - 访问 `/config` 路由查看配置管理演示
   - 展示如何加载和显示配置信息
   - 提供配置热重载功能

#### 配置项说明
- **app**: 应用基本信息（名称、版本、描述）
- **server**: 服务器配置（主机、端口、调试模式）
- **database**: 数据库配置（连接URL、连接池大小）
- **features**: 功能开关（认证、API、WebSocket、缓存）
- **ui**: 用户界面配置（主题、语言、时区）
- **logging**: 日志配置（级别、文件、大小限制）
- **security**: 安全配置（会话超时、登录限制、密码策略）
- **email**: 邮件配置（SMTP设置）
- **storage**: 文件存储配置（类型、路径、文件限制）
- **analytics**: 分析配置（提供商、跟踪ID）

### TailwindCSS 配置
- 配置文件: `tailwind.config.js`
- 输入文件: `styles/input.css`
- 输出文件: `public/styles/output.css`

### Rust 配置
- 主配置: `Cargo.toml`
- 目标: `wasm32-unknown-unknown`
- 输出目录: `public/pkg`

### GitHub Actions 配置
- 工作流文件: `.github/workflows/deploy.yml`
- 权限: `pages: write`, `id-token: write`
- 环境: `github-pages`

## 🚀 性能优化

### 生产环境优化
- WebAssembly 使用 release 模式构建
- TailwindCSS 自动压缩和优化
- 静态资源优化和缓存

### 缓存策略
- GitHub Actions 使用缓存加速构建
- 浏览器缓存静态资源
- Service Worker 支持 (可选)

## 🔍 故障排除

### 常见问题

1. **构建失败**
   ```bash
   # 清理缓存
   cargo clean
   npm run clean
   
   # 重新安装依赖
   npm install
   cargo build
   ```

2. **GitHub Pages 404 错误**
   - 检查 `public/404.html` 文件
   - 确认路由配置正确
   - 验证 base path 设置

3. **WebAssembly 加载失败**
   - 检查 `public/pkg` 目录
   - 验证 MIME 类型配置
   - 确认浏览器支持

### 调试技巧

1. **本地调试**
   ```bash
   # 启用详细日志
   RUST_LOG=debug npm run dev
   ```

2. **GitHub Actions 调试**
   - 查看 Actions 日志
   - 检查权限设置
   - 验证环境变量

## 📚 相关资源

- [Dioxus 官方文档](https://dioxuslabs.com/)
- [Rust WebAssembly 指南](https://rustwasm.github.io/docs/book/)
- [TailwindCSS 文档](https://tailwindcss.com/docs)
- [GitHub Pages 文档](https://pages.github.com/)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

## 🙏 致谢

- [Dioxus](https://dioxuslabs.com/) - 优秀的 Rust UI 框架
- [TailwindCSS](https://tailwindcss.com/) - 实用优先的 CSS 框架 

## 📝 常见问题

- **wasm-pack 未安装**：请先执行 `cargo install wasm-pack`
- **Node.js 依赖未安装**：请先执行 `npm install`
- **serve 版本问题**：请确保 package.json 里 serve 版本为 ^14.2.1
- **构建失败**：请检查 Rust、Node.js、wasm-pack 是否都已正确安装
- **页面样式不生效**：请确认 `public/styles/output.css` 已生成且被 `index.html` 正确引用

## 其他平台

- Linux/macOS 用户可参考 Windows 步骤，使用 `bash build.sh` 替代 `build.bat`

## 其余内容（项目结构、技术栈、组件用法等）详见下方原文档 



```sh
npm install
```
```sh
wasm-pack build --target web

xcopy pkg public\pkg /E /I /Y
```
or
```sh
wasm-pack build --target web --out-dir public/pkg
```
```sh
npx tailwindcss -i ./styles/input.css -o ./public/styles/output.css

# or

npx tailwindcss -i ./styles/input.css -o ./public/styles/output.css --minify
```
```sh
npx serve public
```

```sh
cargo clean

rm public/pkg

rm public/styles/output.css

rm node_modules

npx tailwindcss -i ./styles/input.css -o ./public/styles/output.css --minify

wasm-pack build --target web --out-dir public/pkg

npx serve public -p 8080
```

```sh
cargo clean ; rm public/pkg ; rm public/styles/output.css ; rm node_modules ; npx tailwindcss -i ./styles/input.css -o ./public/styles/output.css --minify ; wasm-pack build --target web --out-dir public/pkg ; npx serve public -p 8080
```


-s 或 --single 选项会让所有 404 路径都 fallback 到 index.html，前端路由就能接管了。

```sh
pnpm build:css:prod

pnpm serve
```