---
title: 本地编码大模型
description: 在本地部署编程大模型的完整实践指南
---

# 本地编码大模型

涵盖本地部署编程大模型的硬件基础、模型选择、性能优化等完整经验。

## 硬件与模型

- [RTX 3090 配置一览](/posts/local-coding-llm-practice) - RTX 3090 显卡详细配置
- [主流编程模型](/posts/local-coding-llm-practice) - DeepSeek-Coder 和 Qwen3-Coder-Next 对比
- [显存瓶颈分析](/posts/local-coding-llm-practice) - GPU 利用率只有 60% 的原因

## 优化方案

- [软件优化](/posts/local-coding-llm-optimization) - 降低 num_ctx、启用 Flash Attention
- [量化格式对比](/posts/local-coding-llm-optimization) - Q4_K_M、Q5_K_M、Q8_0 性能测试
- [上下文长度优化](/posts/local-coding-llm-optimization) - 8000 tokens 的推荐配置
- [常见误区](/posts/local-coding-llm-optimization) - 质量与显存的平衡选择

## 测评对比

- [RTX 3090 编程大模型横评](/posts/rtx3090-llm-benchmark) - 4 个模型横向评测和代码质量对比
- [Qwen3-Coder-Next RTX 3090 优化指南](/posts/qwen3-coder-next-rtx3090) - MoE 模型显存问题与解决方案

## AI 编程实践

- [引导AI从根本上解决Bug](/posts/how-to-debug-and-solve-problems) - 如何让大模型理解代码和bug，避免表面修复
