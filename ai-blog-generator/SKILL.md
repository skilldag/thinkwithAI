---
name: ai-blog-generator
description: >
  AI 生成博客工作流。使用场景：创建博客文章、生成技术文档、调研报告。
  触发词：blog, 文章, 博客, 生成文章, 调研, 文档生成, VitePress.
---

# AI Blog Generator

从调研结果生成博客文章的工作流。

## 核心理念

**问题驱动 + 探究过程 + 拆分/合并审核**

以问题为导向，记录探究过程，最后审核是否需要拆分或合并。

## 工作流程

### 第一步：接收主题

用户输入要生成文章的主题/问题。

### 第二步：搜索调研结果

使用 cdp-scrape skill 搜索相关调研内容：

```bash
# 查看所有已存储的调研文档
doc list

# 搜索相关内容
doc search 关键词
```

如果没有找到相关调研结果，提醒用户先去 deepseek-research 进行调研。

### 第三步：生成文章

使用模板生成文章，确保包含：
- 问题背景：具体场景和痛点
- 探究过程：真实的尝试和追问
- 最终方案：从调研中提取的方案
- 总结：局限性和改进空间

#### 文章结构模板

```markdown
---
title: 问题标题
description: 一句话描述解决的问题和方案
date: YYYY-MM-DD
---

# 问题标题

> 你实际遇到的具体问题是什么？

## 问题背景

- 在什么场景下遇到这个问题
- 之前尝试过什么方法，为什么不 work
- 这个问题困扰你多久

## 探究过程

### 第一次尝试

描述第一次尝试和结果...

### 追问与深入

针对上一次结果的追问...

### 最终方案

从 DeepSeek 调研中提取的完整解决方案

## 总结

- 问题解决了吗？
- 有什么局限性？
- 还可以如何改进？
```

#### 文件命名

- 放在 `docs/posts/` 目录下
- 文件名使用 kebab-case
- front matter 包含 title、description、date

### 第四步：审核拆分/合并

主文章生成后，评估是否需要拆分或合并：

| 情况 | 操作 |
|------|------|
| 内容太多，一篇文章讲不清 | **拆分成多篇**，作为子文章 |
| 只有一个点，三言两语就说完 | **合并到现有文章**，作为补充 |
| 刚刚好，一篇文章讲一个点 | **保持独立** |

判断标准：
- 拆分：涉及 3+ 个独立子问题，或每个子问题都需要大量代码示例
- 合并：核心内容 < 300 字，没有新的实践价值

拆分后的子文章作为主文章的 sidebar 嵌套。

### 第五步：更新配置

添加到 `docs/.vitepress/config.ts` 的 sidebar 配置中。

### 第六步：构建并发布

```bash
# 构建静态文件
pnpm build

# 将构建结果复制到 docs 目录
cp -r docs/.vitepress/dist/* docs/

# 提交并推送
git add -A
git commit -m "feat: 添加文章《标题》"
git push origin gh-pages
```

## 关键要点

- **问题驱动**：从具体问题出发，不是泛泛而谈
- **探究过程**：记录真实的尝试和追问，不是直接给答案
- **拆分/合并审核**：复杂内容拆成子文章，简单内容并入现有文章
- **必须构建**：VitePress 构建后的静态文件在 `docs/.vitepress/dist/`，需要复制到 `docs/` 目录才能部署

## 依赖 Skills

- **cdp-scrape**: 搜索调研结果
- **deepseek-research**: 调研工作流（调研由用户自行完成）
- **opencode-sessions**: OpenCode 会话分析（分析 OpenCode 工作过程）

## 注意事项

- 生成内容放到 `docs/` 下面
- 使用 VitePress 格式
- 确保 sidebar 配置正确
- 验证构建: `pnpm build`
- **重要**：构建后必须将 `docs/.vitepress/dist/` 下的文件复制到 `docs/` 目录再提交

---

## 附加：OpenCode 会话分析工作流

用于分析 OpenCode 会话，生成技术博客文章。

### 使用场景

当用户想要将 OpenCode 使用过程整理成技术文章时使用。

### 工作流程

#### 第一步：确定主题

用户描述想要记录的主题，例如：
- "用 Nim 实现 OpenCode 会话查询工具"
- "OpenCode SQLite 数据库结构分析"

#### 第二步：查询相关会话

使用 opencode-sessions 工具查询相关会话：

```bash
# 列出所有会话（默认 20 条）
opencode-sessions

# 列出更多会话
opencode-sessions --list 50

# 分页查看（从第 20 条开始）
opencode-sessions --offset 20 --limit 30

# 获取会话内容
opencode-sessions <session_id>

# 导出会话到文本文件
opencode-sessions <session_id> -o
```

#### 第三步：提取关键信息

从会话中提取：
- 遇到的问题和挑战
- 尝试的解决方案
- 最终的技术实现
- 代码片段和配置

#### 第四步：生成技术文章

按照文章模板生成，确保包含：
- 场景描述：为什么需要这个工具
- 需求分析：要实现什么功能
- 实现过程：技术选型和代码实现
- 总结：经验教训和后续改进

### 示例主题

- "Nim + SQLite 实现 OpenCode 会话管理工具"
- "OpenCode 数据库结构深度解析"
- "CLI 工具开发实战：从需求到发布"

### 注意事项

- 保持问题驱动的写作风格
- 记录真实的探索过程，包括失败的尝试
- 包含关键代码片段和使用示例
- 可以链接到开源项目或 GitHub 仓库

## OpenCode 会话分析工作流

仅使用 `opencode-sessions` 工具，无需手动执行 `sqlite3`。流程如下：

1. **确定主题** – 例如 `用 Nim 实现 OpenCode 会话查询工具`。
2. **搜索会话** – 通过 `opencode-sessions` 列表或搜索感兴趣的会话。
3. **查看内容** – 使用 `opencode-sessions <session_id>` 直接查看完整对话。
4. **导出文本** – `opencode-sessions <session_id> -o` 导出为 `.txt`，方便后续整理。
5. **生成博客** – 按照 AI Blog Generator 的模板，将对话转化为技术文章。

示例：

```bash
# 列出最近 skilldag 相关会话
opencode-sessions --list 50

# 选择会话
opencode-sessions ses_xxx

# 导出
opencode-sessions ses_xxx -o
```

导出的文件包含完整的 User/Assistant 交互，可直接粘贴进 Markdown 并生成技术文档。
