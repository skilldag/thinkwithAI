---
title: OpenCode 编译与安装指南
description: OpenCode 的多种安装方式和运行方法（含 bunx）
date: 2026-04-09
---

# OpenCode 编译与安装指南

> OpenCode 怎么安装？bunx 怎么用？

## 问题背景

想用 OpenCode，需要安装。但是不确定：
- 安装方式有哪些？
- bunx 运行是怎么回事？
- Oh My OpenCode 怎么安装？

## 探究过程

### 第一次尝试：查看官方安装方式

从会话记录中找到的官方安装方式：

| 安装方式 | 命令 |
|----------|------|
| 安装脚本（推荐） | `curl -fsSL https://opencode.ai/install \| bash` |
| npm | `npm i -g opencode-ai@latest` |
| bun | `bun add -g opencode-ai` |
| pnpm/yarn | `pnpm add -g opencode-ai` |
| Homebrew | `brew install anomalyco/tap/opencode` |
| Windows Scoop | `scoop install opencode` |
| Windows Chocolatey | `choco install opencode` |

### 追问：bunx 是什么？

`bunx` 是 Bun 的包运行器，相当于 npm 的 npx。可以用它直接运行而不需要全局安装：

```bash
# 不需要先安装，直接运行
bunx opencode-ai
bunx opencode-ai --version
```

验证安装：

```bash
opencode --version
```

### Oh My OpenCode 怎么安装？

根据 AGENTS.md，Oh My OpenCode 的安装命令是：

```bash
bunx oh-my-opencode install
```

这需要先安装好 OpenCode 本身。

### 从源码编译

OpenCode 技术栈：
- 核心逻辑：TypeScript + Bun 运行时
- TUI 界面：Go (Bubble Tea)
- 桌面应用：Tauri 2 + SolidJS

**编译步骤：**

```bash
# 1. 克隆仓库
git clone https://github.com/opencode-ai/opencode.git
cd opencode

# 2. 安装 Bun（如果没有）
curl -fsSL https://bun.sh/install | bash

# 3. 安装依赖
bun install

# 4. 编译核心包
cd packages/opencode
bun run build
```

### 国内网络问题

依赖下载慢时，设置镜像：

```bash
# Bun 镜像
bun config set registry https://registry.npmmirror.com

# npm 镜像
npm config set registry https://registry.npmmirror.com
```

## 总结

- **推荐安装**：脚本 `curl -fsSL https://opencode.ai/install | bash`
- **npm/bun 安装**：`npm i -g opencode-ai` 或 `bun add -g opencode-ai`
- **直接运行**：`bunx opencode-ai`（不需要全局安装）
- **Oh My OpenCode**：`bunx oh-my-opencode install`
- **源码编译**：需要 Bun，使用 `bun install && bun run build`
- **国内加速**：设置 npm/bun 镜像

安装完成后运行 `opencode`，然后 `/init` 初始化项目。