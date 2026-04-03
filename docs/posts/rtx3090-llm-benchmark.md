---
title: 本地编程大模型横评：RTX 3090 上跑 Rust 链表哪家强
description: 在 RTX 3090 上测试 qwen3-coder-next、gpt-oss:20b、glm-4.7-flash、minimax-m2.5-free 四个本地大模型的编程能力
date: 2026-04-03
---

# 本地编程大模型横评：RTX 3090 上跑 Rust 链表哪家强

> 在本地跑大模型写代码，体验到底如何？哪个模型既能快速响应，又能生成高质量代码？

## 问题背景

想在自己的 RTX 3090 (24GB) 上运行本地大模型做编程辅助，测试了 4 个主流模型：

| 模型 | 参数量 | 量化 | 大小 |
|------|--------|------|------|
| qwen3-coder-next | 79.7B | Q4_K_M | 51GB |
| gpt-oss:20b | 20.9B | MXFP4 | 13GB |
| glm-4.7-flash | 29.9B | Q4_K_M | 19GB |
| minimax-m2.5-free | - | - | - |

任务很直接：**用 Rust 实现单向链表，包含 insert/delete/print 功能，测试插入 1-5，删除 3，打印结果**。

## 探究过程

### 第一次尝试：GLM 4.7

心想 GLM 口碑不错，先测它。结果：

- GPU 显存飙到 20GB+
- 等待 20 分钟后提示超时
- 挑战失败，直接放弃

### 追问：为什么 GLM 这么慢？

分析发现 glm-4.7-flash 虽然只有 29.9B 参数，但在单卡 3090 上表现不佳，可能是量化方案或架构问题。

### 第二次尝试：gpt-oss:20b

13GB 小模型，期待流畅：

- **生成时间：27秒**
- 第一次生成的代码有错误
- 但模型马上"自愈"并改正了
- 最终编译运行成功，输出正确

### 第三次尝试：qwen3-coder-next

79.7B 大模型，看看能否"一次做对"：

- **生成时间：17分21秒** 😱
- 确实一次生成正确代码
- 但等待时间太长，实际体验差

### 第四次尝试：minimax-m2.5-free

朋友推荐试试：

- **生成时间：25.3秒**
- 一次生成正确代码
- 速度仅比 gpt-oss 慢 2 秒

## 代码质量对比

生成同样功能的链表代码，各模型风格差异明显：

### minimax (84行) ⭐ 最佳

```rust
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
```

- 泛型支持 `LinkedList<T>`
- 正确的 trait bound (`PartialEq`, `Display`)
- 编译期安全，性能最佳

### gpt-oss (71行)

```rust
pub struct LinkedList {
    head: Option<Box<Node>>,
    len: usize,
}
```

- Box 方案，dummy 节点技巧
- 包含 `len` 字段实用
- 代码精炼

### qwen3 (67行)

```rust
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}
```

- 运行时 borrow check
- 过度使用智能指针，内存开销大

### glm (220行) 最差

- 严重过度设计
- 包含 search/size/is_empty/clear 等多余功能
- 不符合"简单链表"需求

## 总结

| 排名 | 模型 | 生成时间 | 代码质量 | 推荐度 |
|------|------|----------|----------|--------|
| 🥇 | minimax-m2.5-free | 25s | 最佳 | ⭐⭐⭐⭐⭐ |
| 🥈 | gpt-oss:20b | 27s | 次佳 | ⭐⭐⭐⭐⭐ |
| 🥉 | qwen3-coder-next | 17min | 一般 | ⭐⭐ |
| 4 | glm-4.7-flash | 超时 | - | ❌ |

### 经验教训

1. **模型不是越大越好**：79.7B 的 qwen3 反而最慢
2. **小模型足够用**：13GB 的 gpt-oss 足够应对简单编程任务
3. **代码质量比速度更重要**：minimax 生成最快且代码最规范

### 局限性与改进

- 只测试了一个简单任务，更全面的评测需要覆盖更多场景
- 未测试长上下文、复杂项目结构等场景
- 可以进一步测试代码优化能力（如性能、内存优化）