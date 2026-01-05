# 微信小程序环境配置管理工具

一款基于 Tauri + Vue 3 + TypeScript 开发的桌面应用，用于管理微信小程序的多环境配置，支持快速切换不同环境的 ext.json 配置文件。

## 主要功能

- **环境管理**：创建、编辑、复制、删除多个环境的配置
- **快速切换**：通过系统托盘菜单快速切换环境，无需打开主窗口
- **配置应用**：一键将环境配置应用到目标项目的 ext.json 文件
- **路径刷新**：批量更新所有环境的文件路径，适合跨项目或跨系统迁移
- **自动备份**：应用配置前自动备份原文件，防止误操作
- **配置同步**：支持导入/导出配置文件，与外部 JSON 文件保持同步
- **智能检测**：自动检测文件内容，避免重复写入相同配置

## 技术栈

- **前端框架**：Vue 3 + TypeScript + Vite
- **桌面框架**：Tauri 2.x
- **UI 组件**：原生 Vue 组件
- **代码编辑器**：CodeMirror 6 (支持 JSON 语法高亮和校验)
- **状态管理**：Pinia
- **路由管理**：Vue Router

## 开发环境

### 推荐的 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 可用命令

| 命令 | 说明 |
|------|------|
| `pnpm run dev` | 只启动 Vite 前端开发服务器，在浏览器中运行 |
| `pnpm run build` | 只构建前端代码，生成静态文件到 `dist/` 目录 |
| `pnpm run preview` | 预览构建后的前端静态文件 |
| `pnpm run tauri:dev` | 启动完整 Tauri 应用（前端 + 桌面窗口），支持热重载 |
| `pnpm run tauri:build` | 构建完整的桌面应用安装包（.exe 等） |

## 使用说明

### 初次使用

1. 启动应用后，点击右上角"添加环境"按钮
2. 填写环境名称（如"开发环境"、"测试环境"）
3. 配置 ext.json 内容
4. 设置目标文件路径（如：`C:\project\miniprogram\src\ext.json`）
5. 点击"应用"按钮将配置写入目标文件

### 切换环境

- **方式一**：在主窗口中点击环境卡片，然后点击"应用"按钮
- **方式二**：右键点击系统托盘图标 → 切换环境 → 选择目标环境（带 ✓ 标记的为当前环境）

### 配置同步

- 在设置页面中可以配置外部 JSON 文件路径
- 导入配置后，所有变更会自动同步到该文件
- 可随时导出配置文件进行备份

### 路径刷新

当切换项目或从其他系统导入配置时，文件路径可能不一致，可使用路径刷新功能：

1. 点击环境列表右上角的"刷新路径"按钮
2. 选择当前项目中的 `ext.json` 文件
3. 系统会自动将所有环境的文件路径更新为选择的文件路径
4. 同时更新全局默认路径

**使用场景**：
- 从 macOS 系统导入配置到 Windows 系统
- 切换到不同的项目目录
- 统一所有环境的文件路径

## 项目结构

```
dsminiEnvSwitch/
├── src/                    # Vue 前端源码
│   ├── components/         # 组件
│   ├── views/             # 页面
│   ├── stores/            # 状态管理
│   ├── types/             # TypeScript 类型定义
│   └── utils/             # 工具函数
├── src-tauri/             # Tauri 后端源码
│   ├── src/
│   │   ├── commands/      # Tauri 命令
│   │   ├── tray.rs        # 系统托盘
│   │   ├── store.rs       # 配置存储
│   │   └── models.rs      # 数据模型
│   └── icons/             # 应用图标
└── dsmini-env-config.json # 默认配置文件
```

## 注意事项

- 目标文件路径请确保有读写权限
- 建议启用自动备份功能，防止配置错误
- 配置文件采用 JSON 格式，请确保语法正确
- 系统托盘菜单支持勾选显示，当前环境会显示 ✓ 标记
- 使用"路径刷新"功能时会更新所有环境的文件路径，请谨慎操作
- 环境状态说明：
  - **当前选中**（紫色）：正在查看的环境
  - **已应用**（绿色）：已应用到文件系统的环境

## 许可证

MIT
