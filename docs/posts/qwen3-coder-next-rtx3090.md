# Qwen3-Coder-Next 在 RTX 3090 上的运行问题与优化指南

记录 Qwen3-Coder-Next 在单卡 RTX 3090 (24GB) 上的运行问题及优化方案。

## 问题背景

用户运行 Qwen3-Coder-Next 模型时，RTX 3090 的 GPU 利用率只有 60%，显存几乎占满（23476MiB / 24576MiB）。

## 问题分析

### 核心原因：MoE 模型显存需求

Qwen3-Coder-Next 是 MoE（混合专家）架构：
- **激活参数**：3B
- **总参数量**：约 80B

MoE 模型的特点：推理时必须将所有专家层加载到显存中，因此实际显存需求远超"激活参数"的字面意思。

| 组成部分 | 估算大小 | 说明 |
|----------|----------|------|
| 模型权重 (Q4_K_M) | ~48GB | 远超单卡 24GB |
| KV Cache | 数GB | 取决于 num_ctx |
| 实际占用 | 23476MiB | 显存几乎满载 |

### 为什么 GPU 利用率只有 60%

当模型无法完全放入 GPU 时，Ollama 采用"部分卸载"策略：
- 一部分层加载到 GPU
- 剩余层在 CPU 上运行

GPU 计算时需要等待 CPU 通过 PCIe 传输数据，导致利用率无法达到 100%。

## 解决方案

### 方案一：软件优化

| 方法 | 说明 |
|------|------|
| 降低 num_ctx | 从默认值降到 4096/8192 |
| 使用更低量化 | Q2_K / Q3_K（牺牲精度） |
| 启用 Flash Attention | v0.11.8+ 默认启用 |

#### 启用 Flash Attention

```bash
# Linux
export OLLAMA_FLASH_ATTENTION=1

# 或编辑 systemd 服务
sudo systemctl edit ollama
# 添加 Environment="OLLAMA_FLASH_ATTENTION=1"
```

效果：显存降低约 15%，推理速度提升约 16%

### 方案二：硬件升级

添加第二张 RTX 3090，使用张量并行（Tensor Parallelism）分配模型层。

### 方案三：使用云端 API

调用提供 Qwen3-Coder-Next 服务的云 API，绕过本地显存限制。

## 总结

- 单卡 24GB 显存不足以完整运行 Qwen3-Coder-Next
- 60% GPU 利用率是显存瓶颈导致的带宽问题
- 优先尝试降低 num_ctx 和启用 Flash Attention
- 根本解决方案是增加显存（双卡或云端）
