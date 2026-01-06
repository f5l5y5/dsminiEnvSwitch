# Gitee + GitHub Actions 自动构建配置指南

## 快速开始（5分钟配置）

### 第一步：创建 GitHub 仓库

1. 访问 https://github.com/new
2. 创建新仓库：
   - 仓库名：`dsmini-env-switch`
   - 可设为私有
   - **不要**初始化 README、.gitignore 等

### 第二步：运行配置脚本

双击运行项目根目录的 `setup-github-mirror.bat`，按提示输入 GitHub 仓库 URL。

### 第三步：同步代码到 GitHub

```bash
# 双击运行
sync.bat
```

或在命令行执行：

```bash
git push github main
```

### 第四步：测试构建

#### 方式 A：通过标签触发（推荐）

```bash
git tag v1.0.0-beta
sync.bat
```

#### 方式 B：手动触发

1. 访问 GitHub 仓库
2. 点击 "Actions" 标签
3. 选择 "Build Tauri App" workflow
4. 点击 "Run workflow"
5. 输入版本号（如 v1.0.0）并运行

### 第五步：下载构建产物

1. 等待构建完成（约 10-15 分钟）
2. 访问 GitHub Actions 页面
3. 点击最近的构建任务
4. 在 "Artifacts" 部分下载对应平台的安装包

## 文件说明

| 文件 | 说明 |
|------|------|
| [.github/workflows/build.yml](.github/workflows/build.yml) | GitHub Actions 配置文件 |
| [setup-github-mirror.bat](setup-github-mirror.bat) | GitHub 镜像仓库配置助手 |
| [sync.bat](sync.bat) | 同步脚本（同时推送到 Gitee 和 GitHub） |
| [BUILD_GUIDE.md](BUILD_GUIDE.md) | 详细的构建指南 |

## 常用命令

### 日常开发流程

```bash
# 1. 提交代码
git add .
git commit -m "feat: 添加新功能"

# 2. 同步到 Gitee 和 GitHub
sync.bat

# 3. 发布新版本
git tag v1.0.0
sync.bat
```

### 查看构建状态

```bash
# 查看所有远程仓库
git remote -v

# 查看 GitHub Actions 状态
gh run list  # 需要安装 GitHub CLI
```

### 仅推送到单一平台

```bash
# 仅推送到 Gitee
git push gitee main

# 仅推送到 GitHub
git push github main
```

## 构建产物说明

### Windows
- **文件格式**：MSI 安装包
- **文件位置**：`src-tauri/target/release/bundle/msi/`
- **GitHub Artifact 名称**：`windows-build-{版本号}`

### macOS
- **文件格式**：DMG 镜像文件
- **文件位置**：`src-tauri/target/release/bundle/dmg/`
- **GitHub Artifact 名称**：`macos-build-{版本号}`

### Linux
- **文件格式**：DEB 包、AppImage
- **文件位置**：`src-tauri/target/release/bundle/`
- **GitHub Artifact 名称**：`linux-build-{版本号}`

## 版本管理建议

### 语义化版本

```
主版本号.次版本号.修订号 (MAJOR.MINOR.PATCH)

例如：
v1.0.0 - 首次发布
v1.0.1 - 修复 bug
v1.1.0 - 添加新功能
v2.0.0 - 重大更新/不兼容的更改
```

### 使用 npm 管理版本

```bash
# 更新修订号（1.0.0 -> 1.0.1）
npm version patch

# 更新次版本号（1.0.0 -> 1.1.0）
npm version minor

# 更新主版本号（1.0.0 -> 2.0.0）
npm version major
```

然后同步：

```bash
sync.bat
```

## 自动化建议

### 1. 设置 Gitee Webhook（可选）

在 Gitee 仓库中配置 Webhook，当代码推送到 Gitee 时自动触发 GitHub Actions。

1. 进入 Gitee 仓库 → 管理 → WebHooks
2. 添加 Webhook：
   - URL：`https://github.com/{username}/{repo}/dispatches`
   - 选择 "Push 事件"

### 2. 使用 GitHub CLI（可选）

安装 GitHub CLI 后，可以更方便地管理：

```bash
# 安装 GitHub CLI
# Windows: winget install --id GitHub.cli
# Mac: brew install gh
# Linux: 参考官方文档

# 登录
gh auth login

# 查看最近的构建
gh run list --limit 5

# 查看构建详情
gh run view

# 重新运行失败的构建
gh run rerun
```

## 故障排查

### 1. GitHub Actions 构建失败

**检查日志**：
- 进入 GitHub 仓库 → Actions
- 点击失败的构建任务
- 查看具体错误信息

**常见问题**：

| 错误 | 解决方案 |
|------|----------|
| `pnpm: command not found` | 检查 workflow 中的 pnpm setup 步骤 |
| `error: linker` not found | 确保 Rust 工具链正确安装 |
| Network timeout | 检查依赖下载，可能需要重试 |

### 2. 同步脚本失败

**检查 GitHub 远程仓库**：
```bash
git remote -v
git remote show github
```

**重新配置 GitHub 远程仓库**：
```bash
git remote remove github
setup-github-mirror.bat
```

### 3. 无法下载构建产物

**检查 Artifact 保留期限**：
- 默认保留 30 天
- 可在 workflow 中调整 `retention-days`

**下载位置**：
- GitHub Actions 页面 → 构建任务 → Artifacts 区域

## 优化建议

### 1. 构建缓存优化

当前已配置以下缓存：
- Rust 构建缓存
- pnpm 依赖缓存
- Node.js 模块缓存

### 2. 构建时间优化

- 初次构建：约 15-20 分钟
- 后续构建：约 5-10 分钟（使用缓存）
- 可通过 `workflow_dispatch` 手动触发测试

### 3. 发布流程优化

创建 `release.bat`：

```batch
@echo off
echo 发布新版本...
set /p VERSION="输入版本号 (例如: v1.0.0): "

git tag %VERSION%
git push gitee --tags
git push github --tags

echo ✅ 版本 %VERSION% 已发布，GitHub Actions 将自动构建
```

## 成本说明

### GitHub Actions 免费额度

| 账号类型 | 免费分钟数/月 | 存储空间 |
|---------|--------------|---------|
| Free | 2000 分钟 | 500 MB |
| Pro | 3000 分钟 | 2 GB |
| Team | 10000 分钟 | 10 GB |

**注意**：
- macOS runner 消耗 10 倍分钟数
- Windows/Linux 消耗 1 倍分钟数
- 免费额度对个人项目足够使用

### 计算示例

每次构建消耗：
- Windows: ~10 分钟
- macOS: ~15 分钟 × 10 = 150 分钟
- Linux: ~10 分钟
- **总计**: 约 170 分钟（macOS）

每月可构建约 11 次（免费账户）

## 下一步

配置完成后，你可以：

1. ✅ 通过标签自动触发构建
2. ✅ 手动触发构建进行测试
3. ✅ 从 GitHub Actions 下载各平台安装包
4. ✅ 创建 GitHub Release 发布新版本
5. ✅ 将下载的安装包上传到 Gitee Releases

## 需要帮助？

- Tauri 官方文档：https://tauri.app/v2/guides/
- GitHub Actions 文档：https://docs.github.com/en/actions
- 本项目详细指南：[BUILD_GUIDE.md](BUILD_GUIDE.md)
