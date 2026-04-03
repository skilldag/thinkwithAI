# 使用 CDP 对 DeepSeek 网页对话数据建档

使用 Chrome DevTools Protocol (CDP) 对 DeepSeek 网页对话进行数据建档，实现调研内容的结构化存储和查询。

## 核心思路

### 1. 数据获取

使用 CDP 连接 DeepSeek 网页版，获取对话数据：

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

CDP 支持查询当前页面可用的操作：

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

## 整合 DeepSeek Research Skill

将 CDP 数据获取能力集成到 DeepSeek Research Skill：

1. `ds add shareurl` → 使用 CDP 提取 DeepSeek 对话内容
2. `ds questions` → 解析对话中的提问，生成思路树
3. `ds topic` → 使用 BM25+ 查询相关调研内容

## 交互设计

### 1. Agent 选择保存内容

在获取对话内容后，Agent 需要决定保存哪些部分：

#### 交互流程

1. **展示选项**: 显示对话中的所有消息块
2. **分析要点**: Agent 提取关键信息
3. **用户确认**: 用户确认保存内容
4. **执行存储**: 保存到本地文档

### 2. Agent 执行登录

当需要登录 DeepSeek 才能访问对话时：

#### 登录步骤

1. **检测状态**: 检查是否需要登录
2. **输入凭证**: 使用用户名密码或扫码
3. **处理验证**: 处理验证码（如有）
4. **等待完成**: 等待页面跳转并确认登录成功

### 3. Agent 决策下一个子页面

在获取当前页面对话后，Agent 决策是否继续获取子页面：

#### 决策逻辑

1. **检测分页**: 检查是否有下一页
2. **评估价值**: Agent 评估继续获取的必要性
3. **用户确认**: 确认是否继续
4. **执行获取**: 导航到下一页并提取内容

## 参考

- [Chrome DevTools Protocol 官方文档](https://chromedevtools.github.io/devtools-protocol/)
- [Niminem/ChromeDevToolsProtocol](https://github.com/Niminem/ChromeDevToolsProtocol)
- [rank_bm25](https://github.com/airblade/pyrankBM25) - BM25 算法实现
