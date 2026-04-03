# RTX 3090 运行 DeepSeek-Coder-33B 模型指南

本文记录了在 RTX 3090 (24GB 显存) 上运行 DeepSeek-Coder-33B 模型的完整指南。

## 概述

本文讨论以下主题：
- RTX 3090 能否运行 DeepSeek-Coder-33B 模型
- 不同量化格式的选择
- 上下文长度的计算

## RTX 3090 能否运行 DeepSeek-Coder-33B

**结论：可以，但需要选择合适的量化版本。**

DeepSeek-Coder-33B 量化版本：
- deepseek-coder-33b-base-AWQ
- deepseek-coder-33b-base-GGUF
- deepseek-coder-33b-base-GPTQ
- deepseek-coder-33b-instruct-AWQ
- deepseek-coder-33b-instruct-GGUF
- deepseek-coder-33b-instruct-GPTQ

### 显存分析

| 量化格式 | 显存占用 | 推荐方案 |
|----------|----------|----------|
| GGUF (q4_k_m) | ~20GB | llama.cpp / Ollama |
| AWQ / GPTQ | ~20GB | vLLM / AutoGPTQ |

### 关键注意事项

- **显存余量紧张**：33B 模型量化后约 20GB，剩余显存有限
- **Base vs Instruct**：instruct 版本更适合对话场景

## 上下文长度计算

### 显存占用模型

- 模型权重：33B 参数，4-bit 量化后约 19-20 GB
- 可用显存：RTX 3090 实际可用约 23.5 GB
- KV cache 公式：4 × 64 × 7168 ≈ 1.75 MB/token

### 不同方案对比

| 方案 | 上下文上限 | 说明 |
|------|------------|------|
| GPTQ/AWQ + float16 KV cache | ~2000 tokens | 实现简单 |
| GGUF + 8-bit KV cache | ~4000 tokens | 平衡方案 |
| GGUF + 4-bit KV cache | ~8000 tokens | **推荐** |
| GGUF + Q3模型 + 4-bit KV cache | ~12000 tokens | 最长上下文 |

### 推荐配置

```bash
./main -m deepseek-coder-33b-instruct.Q4_K_M.gguf \
       -ngl 999 \
       -c 8192 \
       --cache-type-q q4_0
```

## 总结

- RTX 3090 可以运行 DeepSeek-Coder-33B，推荐使用 GGUF 格式 q4_k_m 量化
