---
title: DeepSeek Research Skill 设计思路
description: 如何设计一个使用 DeepSeek 进行深度调研的 OpenCode Skill
date: 2025-01-03
---

# DeepSeek Research Skill 设计思路

> 如何让 DeepSeek 不是直接给答案，而是帮助深入理解一个问题？

## 问题背景

在使用 DeepSeek 进行技术调研时，经常遇到：

- 问一个问题，它给了一个看似完整但很浅的回答
- 不知道它是怎么得出这个结论的
- 调研结果无法复用，下次遇到类似问题又要重新调研

需要一个调研工作流，能够：
1. 保留追问过程，不是只问一次
2. 方便存储和复用调研结果
3. 最终能生成结构化的文档

## 分析与方案

### 第一版：一次性问完整

直接问 DeepSeek："如何配置 VitePress 自动生成 sidebar？"

它给了一个完整的答案，包括代码示例。但实施的时候遇到了问题：
- 它说的方案在具体场景下不 work
- 不知道它是怎么得出这个方案的
- 想追问但对话已经结束了

### 问题出在哪里？

**Q: 一次性问完整为什么会失败？**

A: 它不知道具体场景，给的是通用答案

**Q: 那应该怎么问？**

A: 先问基础方案，再根据回答追问具体问题

**Q: 调研结果如何复用？**

A: 保存 DeepSeek Share Link，提取后聚类存储

### 第二版：追问式调研

后来改变了调研方式：

1. 先问基础问题，获取初步回答
2. 针对回答中的要点继续追问
3. 至少追问 2-3 轮，获取深度内容
4. 保存 Share Link，方便后续复用

每次追问，都让 DeepSeek 解释：
- 为什么这样做？
- 在什么场景下不适用？
- 有没有更好的方案？

### 最终方案

设计 DeepSeek Research Skill，核心功能：

1. **追问式调研**：至少 2-3 轮追问，获取深度内容
2. **Share Link 存储**：保存调研对话，方便追溯
3. **聚类与查询**：使用 cdp-scrape 整理和搜索

核心命令：

| 命令 | 说明 |
|------|------|
| `doc add "url"` | 添加 DeepSeek Share 到聚类文档 |
| `doc list` | 查看所有已存储的调研文档 |
| `doc search 关键词` | 搜索相关内容 |

### OpenCode Skill 基础

要理解这个 Skill，需要先知道如何创建 OpenCode Skill：

1. 在 `~/.agents/skills/` 目录下创建新文件夹
2. 添加 `SKILL.md` 文件
3. 使用 YAML front matter 定义 name 和 description

```yaml
---
name: skill-name
description: >
  技能描述。使用场景关键词（触发词）。
---

# 技能标题

技能详细内容...
```

当用户提问匹配 description 中的关键词时，opencode 会自动加载对应的 skill。

## 总结

- 问题解决了吗？调研过程更系统化了
- 有什么局限性？需要手动保存 Share Link
- 还可以如何改进？考虑自动保存调研过程

相关阅读：
- [AI Blog Generator Skill](/posts/ai-blog-generator) - 生成博客的工作流
- [AI 生成博客工作流](/posts/ai-generated-blog-workflow) - 整体工作流
