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

# 安装过程

本文探索了三种不同的 LLM 推理服务：**Ollama**、**vLLM** 和 **sglang**。下面分别介绍每种服务的安装方式和配置方法。

## Ollama 安装

Ollama 是最简单的部署方式，官方提供了一个一键安装脚本。

### macOS 安装

```bash
# 使用官方脚本安装
curl -fsSL https://ollama.com/install.sh | sh

# 验证安装
ollama --version
```

### Linux 安装

```bash
# 使用官方脚本安装
curl -fsSL https://ollama.com/install.sh | sh

# 验证安装
ollama --version
```

### 配置 Modelfile

首次使用前，需要创建 Modelfile 来定义模型：

```bash
# 创建 Modelfile
mkdir -p ~/.ollama
vim ~/.ollama/Modelfile
```

```bash
FROM glm://glm-4.7-flash
PARAMETER num_ctx 128000
PARAMETER num_gpu_layers 35
PARAMETER temperature 0.1
```

### 启动服务

```bash
# 启动 Ollama 服务
ollama serve

# 运行模型
ollama run glm-4.7-flash
```

### 特点总结

| 优点 | 缺点 |
|------|------|
| ⚡ 安装最简单 | ⚠️ 推理性能不是最优 |
| 🚀 资源消耗低 | ⚠️ 功能相对简单 |
| 💡 本地运行友好 | ⚠️ 扩展性有限 |

---

## vLLM 安装

vLLM 是基于 PagedAttention 的高性能推理服务，适合生产环境。

### 环境要求

- Python 3.8+
- CUDA 支持（GPU 环境）

### 安装方法

```bash
# 使用 pip 安装
pip install vllm

# 或者安装最新版本
pip install --upgrade vllm
```

### 配置方法

创建 `config.json` 文件：

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

### 启动配置

```bash
# 基础启动
vllm serve --model your-model-path/your-model

# 高级配置
vllm serve --model your-model-path/your-model \
  --gpu-memory-utilization 0.9 \
  --trust-remote-code \
  --max-model-len 65000
```

### 特点总结

| 优点 | 缺点 |
|------|------|
| 🚀 推理性能提升 24× | ⚠️ 配置相对复杂 |
| ⚡ 优化 PagedAttention | ⚠️ 需要较高的 GPU 要求 |
| 🏭 生产环境友好 | ⚠️ 上下文限制明显 |
| 📈 吞吐量高 | ⚠️ 内存占用较高 |

---

## sglang 安装

sglang 是类似 vLLM 的高性能推理服务，强调动态批处理和算子融合。

### 环境要求

- Python 3.8+
- CUDA 支持（GPU 环境）

### 安装方法

```bash
# 使用 pip 安装
pip install "sglang[all]"

# 或者分步安装
pip install sglang
pip install auto-gptq
pip install einops
```

### 配置方法

创建 `serve_config.py` 文件：

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

### 启动配置

```bash
# 命令行启动
python -m sglang.launch_server \
  --model-path your-model-path \
  --port 30000 \
  --gpu-memory-utilization 0.9

# 或者使用 Python 脚本启动
python serve_config.py
```

### 特点总结

| 优点 | 缺点 |
|------|------|
| ⚡ 动态批处理 | ⚠️ 依赖配置复杂 |
| 🎯 算子融合优化 | ⚠️ awq 兼容性问题 |
| 🔄 企业级部署灵活 | ⚠️ 调试难度较高 |
| 💰 性价比高 | ⚠️ 模型格式要求严格 |

---

## 安装总结对比

| 服务 | 安装难度 | 配置复杂度 | 资源要求 | 适合场景 |
|------|---------|-----------|---------|---------|
| Ollama | ⭐ 简单 | ⭐ 直观 | 低 | 个人开发、学习 |
| vLLM | ⭐⭐ 中等 | ⭐⭐ 复杂 | 高 | 生产环境、高吞吐 |
| sglang | ⭐⭐⭐ 复杂 | ⭐⭐⭐ 较复杂 | 中-高 | 企业级、高性能 |

> 💡 **建议**：对于本地开发和学习，推荐从 **Ollama** 开始。如果需要生产级性能，再考虑使用 **vLLM** 或 **sglang**。

## 实际使用情况

### 为什么没有使用 sglang？

在尝试使用 sglang 时遇到以下关键技术问题：

1. **依赖版本不兼容**：
   - sglang 依赖 `awq` 量化库的特定版本
   - 当前的 `awq` token-compresser 与 sglang 不完全兼容
   - 导致模型加载失败或推理过程频繁崩溃

2. **模型格式问题**：
   - 没有找到比 `glm-4.7-flash` 更好的模型格式
   - 已尝试的量化格式（AWQ、GPTQ）都无法正常工作
   - 原始格式显存占用过高，无法在本地部署

3. **性能问题**：
   - 由于兼容性问题，推理性能下降明显
   - 错误日志多，调试困难

**结论**：sglang 的依赖问题无法解决，且没有替代方案，因此放弃使用。

### 为什么没有使用 vLLM？

在尝试使用 vLLM 时遇到以下限制：

1. **Context 限制问题**：
   - 尝试将上下文窗口配置为 128k
   - **实际上无法越过 65k 的 context 限制**
   - 当对话超过 65k tokens 时，vLLM 会抛出错误或截断对话

2. **配置陷阱**：
   - `context_length` 参数只影响可调用模型的最大 tokens
   - `num_gpu_blocks` 等参数无法解决实际的上下文窗口限制
   - 文档中提到此限制的说明不够清晰

3. **OOM 问题**：
   - 增加上下文窗口时会迅速耗尽 GPU 显存
   - 即使配置了 128k，实际可用只有约 65k

**结论**：vLLM 的上下文限制对于长对话场景来说是不够的，因此选择 Ollama 作为替代方案。

### 为什么最终选择 Ollama？

经过实际测试和对比，最终选择 **Ollama** 作为推理服务：

| 评估维度 | Ollama | vLLM | sglang |
|---------|--------|------|--------|
| 安装难度 | ✅ 一键安装 | ⚠️ pip 安装，依赖多 | ⚠️ pip 安装，复杂依赖 |
| 上下文限制 | ✅ 支持 128k | ❌ 仅约 65k | ⚠️ 受模型格式限制 |
| 配置灵活度 | ✅ 简单直观 | ⚠️ 参数多，配置复杂 | ⚠️ Python 配置繁琐 |
| 依赖兼容性 | ✅ 稳定 | ⚠️ 有版本冲突 | ❌ awq 不兼容 |
| 推理性能 | ✅ 足够使用 | ✅ 高性能 | ✅ 理论上高性能 |
| 本地部署友好度 | ✅ 完美 | ⚠️ 需要专业 GPU | ⚠️ 资源消耗高 |
| 故障排查 | ✅ 日志清晰 | ⚠️ 日志复杂 | ❌ 调试困难 |

**最终结论**：在本地开发场景下，Ollama 的上下文支持、安装难度和稳定性综合最佳。

## OpenCode 集成

### 配置示例（推荐使用 Ollama）

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
