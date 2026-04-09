---
title: OpenCode 源码编译与使用问题汇总
description: 记录 OpenCode 源码编译过程及使用中遇到的问题
date: 2026-04-09
---

# OpenCode 源码编译与使用问题汇总

> 从会话记录中整理的 OpenCode 使用经验教训

## 问题背景

最近使用 OpenCode 时遇到了多个问题，从源码编译到实际使用中的各种坑。整理如下：

---

## 1. OpenCode 源码编译安装

### 1.1 环境要求

- **Bun 1.3+**
- Git

### 1.2 编译步骤

```bash
# 1. 克隆仓库
git clone https://github.com/anomalyco/opencode.git
cd opencode

# 2. 安装依赖
bun install

# 3. 构建独立可执行文件
./packages/opencode/script/build.ts --single

# 4. 复制到目标位置
cp ./packages/opencode/dist/opencode-darwin-arm64/bin/opencode /Users/meetai/.opencode/bin/opencode
```

### 1.3 遇到的问题：node_modules 冲突

**问题现象**：
- 直接运行编译出的 binary 正常：`/Users/meetai/source/nopencode/ts-opencode/packages/opencode/dist/opencode-darwin-arm64/bin/opencode --version` ✓
- 复制到目标位置后运行被 killed：`/Users/meetai/.opencode/bin/opencode --version` ✗ (killed, exit code 137)

**排查过程**：
1. 比较两个 binary — 完全相同
2. 使用 `env -i` 纯净环境测试 — 可以正常运行
3. 检查用户 PATH 环境变量 — 看起来正常
4. 检查 `/Users/meetai/.opencode/` 目录 — 发现有旧的 `node_modules/`

**根本原因**：
旧版本的 OpenCode 是 Node.js 版本，安装在 `/Users/meetai/.opencode/` 目录下，里面有 `node_modules`。新编译的 binary 会意外加载这些 node_modules 导致冲突。

**解决方案**：
```bash
rm -rf /Users/meetai/.opencode/node_modules
```

然后正常运行。

---

## 2. 使用 Rust 写链表算法

### 2.1 任务描述

用户要求用 Rust 写一个链表算法并测试验证。

### 2.2 遇到的问题：项目结构不匹配

**问题现象**：
- 直接运行 `cargo test` 报错：`could not find Cargo.toml`
- 因为代码是直接写入文件的，不是放在 Cargo 项目中

**解决过程**：
1. 尝试 `cargo init` — 因为目录名是 `test`（Rust 保留字）而失败
2. 使用 `cargo new --lib linked_list_project` 创建正确的项目结构
3. 将代码写入 `src/lib.rs`
4. 运行 `cargo test`

### 2.3 实现的测试用例

| 测试用例 | 验证内容 |
|---------|---------|
| `test_new_and_is_empty` | 新建空链表 |
| `test_push_front_single_element` | 单元素头部插入 |
| `test_push_front_multiple` | 多元素头部插入 |
| `test_push_back_to_empty` | 单元素尾部插入 |
| `test_push_back_multiple` | 多元素尾部插入 |
| `test_pop_front` | 头部删除 |
| `test_mixed_operations` | 混合操作生命周期 |
| `test_reverse_list` | 链表反转 |

---

## 3. 模型中途停止问题

### 3.1 用户反馈

用户问：为什么 gemma4 这个模型，每次不把任务执行完毕就停止了？

### 3.2 分析

这个问题涉及到模型的推理能力边界和任务分解能力。从会话记录来看，模型在处理复杂多步骤任务时，可能会：
- 遇到困难时过早停止
- 没有完成所有 todo 就返回结果
- 对任务复杂度的估计不足

---

## 4. Oh My OpenCode 插件

### 4.1 Easter Egg

在一次会话中触发了 Oh My OpenCode 的 Easter Egg：

> **Oh My OpenCode** 是一个强大的 OpenCode 插件，将 AI agent 转变为完整的开发团队：
> - Multi-Agent Orchestration: Oracle, Librarian, Explore, Frontend Engineer 等
> - LSP Tools: 完整的 IDE 能力
> - AST-Grep: 25 种语言的代码搜索
> - Background Agents: 并行运行多个 agent

### 4.2 安装命令

```bash
bunx oh-my-opencode install
```

### 4.3 与 Build Agent 的关系

用户问：oh-my-opencode 是否会自动从 Build agent 切换到 oh-my-opencode 的 agent？

根据代码分析：
- `OpenCode-Builder` 是 Claude Code 内置的 Build agent
- oh-my-opencode 有一个隐藏的 build agent 配置 (`hidden: true`, 模式为 "subagent")
- 关键词检测器会跳过 OpenCode-Builder 的关键词注入

---

## 5. 配置模型

### 5.1 配置文件位置

OpenCode 配置文件：`~/.opencode/opencode.json`

### 5.2 模型配置示例

```json
{
  "models": {
    "default": "anthropic/claude-sonnet-4-20250514",
    "coding": "openai/gpt-4"
  },
  "providers": {
    "ollama": {
      "url": "http://192.168.3.10:11434"
    }
  }
}
```

---

## 6. Plan Mode 下的约束

在 Plan Mode 下，agent 处于只读阶段：
- **禁止**：任何文件编辑、修改、系统变更
- **要求**：只能提问、询问用户意见
- **目的**：构建计划，而不是执行

用户问"安装成功吗"，在 Plan Mode 下无法直接回答，需要询问具体是哪个安装。

---

## 总结

| 问题 | 解决方案 |
|------|----------|
| 编译后 binary 被 killed | 删除旧的 node_modules 目录 |
| cargo test 失败 | 创建正确的 Cargo 项目结构 |
| 模型中途停止 | 需要更好的任务分解策略 |
| Plan Mode 只读 | 只能提问，不能执行 |

---

## 相关命令速查

```bash
# 安装 oh-my-opencode
bunx oh-my-opencode install

# 编译 OpenCode
bun install
./packages/opencode/script/build.ts --single

# 清理冲突的 node_modules
rm -rf ~/.opencode/node_modules
```