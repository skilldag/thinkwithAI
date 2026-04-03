---
title: OpenCode Sessions CLI：从查询到导出完整会话
description: 介绍如何使用 opencode-sessions 工具在 OpenCode 环境中快速提取、导出对话，适配博客生成工作流。
date: 2026-04-03
---

# OpenCode Sessions CLI

> 在 OpenCode 中，**会话** 记录了我们的所有交流，包含了调试、设计、实现等关键细节。想要复盘或分享这些内容，最直观的方式是 **导出完整会话**，随后按 Markdown 模板整理为技术博客。

## 目标

- 简单查询最近会话或按关键词搜索
- 获取完整的 `session → message → part` 结构
- 自动导出为可直接粘贴 Markdown 的文本文件

## 基础使用

```bash
# 列出最近 20 条会话
opencode-sessions

# 列出更多（如 50 条）
opencode-sessions --list 50

# 分页查看（从第 20 条开始）
opencode-sessions --offset 20 --limit 30

# 查看指定会话
opencode-sessions <session_id>

# 导出会话到文件
opencode-sessions <session_id> -o
```

> 只需运行上述命令，即可得到类似 `ses_2ad862341ffe3TEYSzBCwAIyV3.txt` 的完整对话文件，文件内按 `role | content` 逐行记录。

## 工作流程示例

1. **确定主题**：例如 `用 Nim 实现 OpenCode 会话查询工具`。
2. **查找相关会话**：
   ```bash
   opencode-sessions --list 50
   ```
3. **获取会话**：
   ```bash
   opencode-sessions ses_2ad862341ffe3TEYSzBCwAIyV3
   ```
4. **导出**：
   ```bash
   opencode-sessions ses_2ad862341ffe3TEYSzBCwAIyV3 -o
   ```
5. **写博客**：按 AI Blog Generator 的模板，将导出的文本转化为 Markdown。

## 技术细节

- **Nim + SQLite3 CLI**：避免编译时依赖系统 SQLite，直接在命令行执行查询。
- **JSON 结构**：对话内容在 `part` 表中以 `json` 存储，`text` 字段保存实际文本。
- **跨平台**：脚本仅使用 POSIX 系统通用工具，可在 macOS、Linux 直接使用。

## 代码示例（核心）

```nim
import osproc

proc runSQLite(sql: string): string =
  result = execProcess("sqlite3", params = ["$HOME/.local/share/opencode/opencode.db", sql])
```

后续实现通过 `osproc` 调用 `sqlite3` 并解析 JSON，生成 `role | content` 格式输出。

## 结语

使用 `opencode-sessions`，你即可在数秒内获得完整会话记录，再配合 AI Blog Generator 的模板，轻松把日常技术记录转化为可发布的博客。祝你写作愉快 🚀
