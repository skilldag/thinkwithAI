---
title: OpenCode + Ollama 上下文管理问题
description: GLM-4.7-flash 模型"失忆"的原因和解决方案
date: 2026-04-14
---

# OpenCode + Ollama 上下文管理问题

## 背景

用户在使用 OpenCode 和 Ollama 运行 GLM-4.7-flash 模型时，经常遇到模型忘记对话上下文的问题：

> "opencode ollama glm-4.7-flash 模型，经常不知道第一个问题是啥，以为自己是刚开始对话"

这是一个典型的"失忆"问题。

## 分析

### 根本原因：模型的无状态特性

大语言模型本质上是无状态的。它们不会自动记住之前的对话，每次都需要把完整的对话历史和当前问题一起交给它。

可以理解为：模型像一个记忆力很差但知识渊博的专家。需要你每次都像"背景资料"一样提供完整上下文。

### 上下文窗口机制

**GLM-4.7-flash 在 Ollama 中的默认上下文窗口为 128k tokens**，理论上可以处理很长的对话。

但在实际使用中，有两点需要注意：

1. **模型原生上下文 vs OpenCode 管理**
   - 128k 是模型在 Ollama 下的理论上限
   - OpenCode 作为客户端，有自己的一套上下文管理策略（如超限自动压缩）

2. **内存占用问题**
   - 长期对话中，OpenCode 消耗的内存可能远超模型原生上下文
   - 当会话超出限制时，OpenCode 会启动压缩机制

### 压缩的双刃剑

压缩机制可以缓解超限问题，但会丢失细节：

- 当会话被压缩成摘要后，原文内容被丢弃
- 当你再次问"第一个问题是啥"时，模型只能根据摘要回忆，给不出原文
- 这就是"失忆"的真正原因

# 对比分析 - 不同推理服务的配置与实践

## 概述

当选择 GLM-4.7-flash 作为推理服务时，主要有三种选择：
- **Ollama**：简化部署，本地开发友好
- **vLLM**：高性能推理，优化 PagedAttention 机制
- **sglang**：类似 vLLM 的高性能推理服务

## 配置对比

| 特性 | Ollama | vLLM | sglang |
|------|--------|------|--------|
| 部署难度 | ⭐ 简单 | ⭐⭐ 中等 | ⭐⭐ 中等 |
| 启动命令 | `ollama serve` | `vllm serve` | `python -m sglang.launch_server` |
| 配置文件 | `~/.ollama/Modelfile` | `config.json` | Python 配置 |
| API 端口 | 11434 | 8000 | 默认 30000 |
| 性能特点 | 平衡 | 优化吞吐量 | 动态批处理 |
| 适用场景 | 本地开发、快速验证 | 生产环境、高吞吐量 | 企业级部署、高性能 |
| 支持 GPU | ✅ | ✅ | ✅ |
| 模型导入 | `ollama pull` | 自定义目录 | HuggingFace/本地加载 |

## 性能对比

### 推理性能

- **vLLM**：基于 PagedAttention 策略，优化内存管理，推理吞吐量提升 24×
- **sglang**：动态批处理和算子融合，减少 GPU 计算闲置，性价比高
- **Ollama**：资源消耗低，适合本地实验和学习

### 资源消耗对比

| 服务 | 显存占用 | CPU 占用 | 网络开销 | 适用 GPU |
|------|---------|---------|---------|---------|
| Ollama | 中 | 低 | 无 | 消费级 GPU |
| vLLM | 高 | 中 | 无 | 专业 GPU |
| sglang | 中-高 | 中 | 无 | 专业 GPU |

## 适用场景

### Ollama 适合

- 🔧 本地开发和快速验证
- 📚 学习和实验推理服务
- 💻 开发者个人工作站
- 🔒 需要完全离线运行

**典型配置**：
- 上下文窗口：16k-128k
- 模型：轻量级模型优先
- 并发：1-2 个并发请求

### vLLM 适合

- 🏭 生产环境部署
- 📈 高吞吐量需求场景
- 💼 企业级应用
- ⚡ 需要极致推理性能

**典型配置**：
- 上下文窗口：32k-128k
- 批处理：并发请求优化
- 部署：Docker/Kubernetes 上

### sglang 适合

- 🏢 企业级高性能部署
- 🎯 复杂的工作负载
- ⚖️ 灵活的资源调度
- 🔥 对性价比要求高

**典型配置**：
- 上下文窗口：64k-128k
- 自定义硬件优化
- 监控和可观测性工具

## 配置示例

### Ollama 配置

创建 `~/.ollama/Modelfile`：

```bash
# Modelfile
FROM glm://glm-4.7-flash
PARAMETER num_ctx 128000
PARAMETER num_gpu_layers 35
PARAMETER temperature 0.1
```

启动服务：

```bash
ollama serve
```

### vLLM 配置

创建 `config.json`：

```jsonc
{
  "model": "your-model-path/your-model",
  "tensor_parallel_size": 1,
  "gpu_memory_utilization": 0.9,
  "num_gpu_blocks": 1000,
  "num_caller_blocks": 1000,
  "trust_remote_code": true
}
```

启动服务：

```bash
vllm serve --model your-model-path/your-model \
  --gpu-memory-utilization 0.9 \
  --trust-remote-code
```

### sglang 配置

创建 `serve_config.py`：

```python
from sglang import Server

server = Server(
    model_path="your-model-path",
    port=30000,
    gpu_memory_utilization=0.9,
    trust_remote_code=True
)
server.launch()
```

启动服务：

```bash
python -m sglang.launch_server \
  --model-path your-model-path \
  --port 30000 \
  --gpu-memory-utilization 0.9
```

## OpenCode 集成

### 配置示例

在 `~/.opencode/opencode.json` 中配置：

```json
{
  "provider": "ollama",
  "baseURL": "http://192.168.3.10:11434/v1",
  "ollama": {
    "model": "glm-4.7-flash-100k",
    "numCtx": 100000
  }
}
```

```json
{
  "provider": "vllm",
  "baseURL": "http://192.168.3.10:8000/v1",
  "llm": {
    "model": "glm-4.7-flash",
    "context_length": 128000
  }
}
```

```json
{
  "provider": "sglang",
  "baseURL": "http://192.168.3.10:30000/v1",
  "llm": {
    "model": "glm-4.7-flash",
    "max_tokens": 4096
  }
}
```

## 总结

选择推理服务时需要权衡：

| 场景 | 推荐 | 理由 |
|------|------|------|
| 个人开发者 | Ollama | 简单易用，资源消耗低 |
| 生产环境 | vLLM | 高性能，成熟稳定 |
| 企业部署 | sglang | 高性价比，灵活性好 |

> 💡 **建议**：从 Ollama 开始，随着需求增长考虑 vLLM 或 sglang。根据实际使用场景和资源情况选择最适合的方案。

## 解决方案

### 方案一：升级 Ollama 配置（根本解决）

增加 Ollama 的上下文窗口，避免触发压缩机制：

```bash
# 进入 Ollama 对话
ollama run glm-4.7-flash

# 设置上下文窗口大小
/set parameter num_ctx 32768

# 保存新配置
/save glm-4.7-flash-16k
```

**建议**：根据实际需求调整 `num_ctx` 数值，常见选择：16k、32k、64k

### 方案二：手动管理对话记忆（临时方案）

在每次提问时，主动把之前的关键结论作为背景信息：

**开启新对话时**：
```
当前任务背景：我要实现一个用户认证系统，使用 JWT，需要支持 refresh token 轮换
```

**当模型开始"犯糊涂"时**：
```
关键回顾：最初的任务是实现用户认证，使用 JWT 和 refresh token 轮换机制
```

## 总结

这不是特定模型的缺陷，而是 AI 对话系统的通用设计。关键在于：

1. **理解模型的无状态本质** - 需要主动管理上下文
2. **理解压缩机制的影响** - 避免依赖摘要内容作为关键信息
3. **选择合适的工具** - 长对话优先使用方案一的配置升级，短期对话可用方案二手动管理

> 主动、有技巧地管理 AI 的"短期记忆"，才能发挥其最大价值。
