# GitHub Actions 工作流说明

本项目包含以下 GitHub Actions 工作流：

## 工作流文件

### 1. `build.yml` - 构建和部署
- **触发条件**: 推送到 main/master 分支或创建 PR
- **功能**: 
  - 构建 TailwindCSS 样式
  - 编译 WebAssembly
  - 自动部署到 GitHub Pages
- **部署**: 仅在 main/master 分支推送时部署
- **权限**: 使用 GitHub Pages 官方 Actions，无需额外配置

### 2. `test.yml` - 测试构建
- **触发条件**: 推送到 main/master/develop 分支或创建 PR
- **功能**:
  - 验证 Rust 代码编译
  - 构建 TailwindCSS
  - 构建 WebAssembly
  - 验证构建输出
- **用途**: 用于 PR 检查和开发分支验证

### 3. `windows.yml` - Windows 构建
- **触发条件**: 推送到 main/master 分支或创建 PR
- **功能**:
  - 在 Windows 环境下构建
  - 使用 Windows 特定的文件复制命令
  - 上传构建产物
- **用途**: 跨平台兼容性测试

## 使用方法

1. **启用 GitHub Pages**:
   - 进入仓库设置 → Pages
   - Source 选择 "GitHub Actions"
   - 确保仓库有 Pages 权限

2. **推送代码**:
   ```bash
   git add .
   git commit -m "Add GitHub Actions workflows"
   git push origin main
   ```

3. **查看工作流**:
   - 进入仓库的 Actions 标签页
   - 查看工作流运行状态

## 权限配置

工作流使用了以下权限：
- `contents: read` - 读取仓库内容
- `pages: write` - 写入 GitHub Pages
- `id-token: write` - 用于身份验证

## 注意事项

- ✅ 使用官方 GitHub Pages Actions，无需手动配置 token
- ✅ 自动处理权限问题
- ✅ 支持并发部署控制
- ✅ 构建产物会自动上传到 GitHub Pages
- ✅ 可以通过 Actions 标签页下载构建产物进行本地测试

## 故障排除

如果遇到权限错误：
1. 确保仓库设置中启用了 GitHub Pages
2. 检查仓库的 Actions 权限设置
3. 确保工作流文件中的权限配置正确

## 自定义

如需修改工作流：
1. 编辑对应的 `.yml` 文件
2. 提交并推送更改
3. 工作流会自动重新运行 