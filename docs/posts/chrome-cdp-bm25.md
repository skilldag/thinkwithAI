---
title: Chrome CDP 与 BM25 数据建档
description: 使用 Chrome DevTools Protocol 获取数据与 BM25 索引检索的完整方案
---

# Chrome CDP 与 BM25 数据建档

使用 Chrome DevTools Protocol (CDP) 对网页数据进行数据建档，配合 BM25 进行结构化存储和查询。

## 核心思路

### 1. 数据获取

使用 CDP 连接网页版 DeepSeek，获取对话数据：

```nim
import cdp

let browser = await launchBrowser()
let tab = await browser.newTab()

# 导航到 DeepSeek 对话页面
discard await tab.navigate("https://chat.deepseek.com")

# 获取对话内容
let messages = await tab.evaluate("""
  Array.from(document.querySelectorAll('.message')).map(msg => ({
    role: msg.querySelector('.role')?.textContent,
    content: msg.querySelector('.content')?.textContent
  }))
""")
```

#### Agent 可操作列表

| 操作 | CDP 命令 | 说明 |
|------|----------|------|
| 获取页面 DOM | DOM.getDocument | 获取完整 DOM 树 |
| 执行 JS | Runtime.evaluate | 获取对话消息 |
| 监听网络 | Network.enable | 拦截 API 请求 |
| 页面截图 | Page.captureScreenshot | 可视化调试 |

### 2. 数据建档

获取对话数据后，使用 BM25+ 进行索引和检索：

#### 数据结构

```json
{
  "id": "uuid",
  "timestamp": "2024-01-01T00:00:00Z",
  "topic": "主题",
  "messages": [
    {
      "role": "user",
      "content": "问题内容",
      "timestamp": "..."
    },
    {
      "role": "assistant",
      "content": "回答内容",
      "links": ["参考链接"]
    }
  ]
}
```

#### BM25+ 索引

BM25+ 是一种改进的全文检索算法，适合长文本检索：

```python
from rank_bm25 import BM25Okapi

corpus = [msg["content"] for msg in messages]
bm25 = BM25Okapi(corpus)

# 查询
query = "用户问题"
scores = bm25.get_scores(query)
top_results = bm25.get_top_n(query, corpus, n=3)
```

## 工作流程

### Step 1: 启动浏览器

```bash
chrome --remote-debugging-port=9222 --headless
```

### Step 2: 连接 CDP

```nim
let browser = await launchBrowser()
let tab = await browser.newTab()
```

### Step 3: 获取对话

```nim
# 获取所有消息
let messages = await extractMessages(tab)

# 提取参考链接
let links = await extractLinks(tab)
```

### Step 4: 存储建档

```python
# 保存到本地
save_to_json(messages, "conversation.json")

# 建立 BM25 索引
index_messages(messages)
```

### Step 5: 查询检索

```python
# BM25 查询
results = query("调研主题", top_k=5)
```