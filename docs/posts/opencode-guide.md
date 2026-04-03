# OpenCode 全面指南

记录 OpenCode 的使用方法、Skills 创建和存储结构。

## 什么是 OpenCode

OpenCode 是一个 AI 辅助编程 CLI 工具，帮助用户进行软件工程任务。

## OpenCode 存储结构

### 目录概览

```
~/.local/share/opencode/
├── opencode.db   # 736M - SQLite 数据库，存储会话、消息等
├── snapshot/    # 1.5G - 代码快照库
├── storage/     # 144M - 附件、嵌入向量等
└── tool-output/ # 9.4M - 工具命令输出缓存
```

### opencode.db

存储所有结构化数据：
- 会话记录
- 消息内容
- 用户设置

### snapshot/ - 代码快照

存储项目代码的历史快照，用于让 AI 理解代码变更：

```
snapshot/
  <project-id>/     # 项目标识
    <snapshot-id>/  # 快照 ID（类似 Git 仓库）
      HEAD, objects/, refs/
```

## 创建 OpenCode Skills

### 步骤

1. 在 `~/.agents/skills/` 目录下创建新文件夹
2. 在文件夹中添加 `SKILL.md` 文件
3. 使用 YAML front matter 定义 name 和 description

### SKILL.md 格式

```yaml
---
name: skill-name
description: >
  技能描述。使用场景关键词（触发词）。
---

# 技能标题

技能详细内容...
```

### 配置项

| 配置项 | 说明 |
|--------|------|
| name | 技能唯一标识符 |
| description | 包含触发关键词，用户提问匹配时会加载此 skill |

## DeepSeek Research Skill

使用 DeepSeek 进行深度调研的工作流。

### 核心命令

| 命令 | 说明 |
|------|------|
| `ds add shareurl "https://xxx"` | 添加 DeepSeek Share 到聚类文档 |
| `ds questions` | 查询调研过程的对话所有提问，理解用户调研思路，整理成思路树 |
| `ds topic "abc"` | 查询某个主题的相关内容 |

## CDP 自动化 Skill

使用 Chrome DevTools Protocol (CDP) 进行浏览器自动化。

### 常用域

| 域 | 功能 |
|----|------|
| Page | 页面导航、截图 |
| Runtime | JavaScript 执行 |
| Network | 网络拦截 |

### 工作流程

1. 启动浏览器：`chrome --remote-debugging-port=9222 --headless`
2. 连接 CDP
3. 执行操作
4. 关闭连接

## AI Blog Generator Skill

自动化从 AI 调研结果生成博客文章的工作流。

### 工作流程

1. **确定主题** - 与用户确认文章标题、目标受众
2. **调研过程** - 获取 DeepSeek 调研内容
3. **生成文档** - 生成 VitePress 格式 Markdown
4. **提交部署** - 推送到 gh-pages

### 父子文章关系

VitePress 支持嵌套 sidebar：

```typescript
sidebar: [
  {
    text: 'Posts',
    items: [
      { 
        text: 'AI Generated Blog Workflow', 
        link: '/posts/ai-generated-blog-workflow',
        items: [
          { text: '如何创建 OpenCode Skills', link: '/posts/create-opencode-skills' },
          { text: 'AI Blog Generator Skill 设计思路', link: '/posts/ai-blog-generator' }
        ]
      }
    ]
  }
]
```
