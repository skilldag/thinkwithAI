
### 方案一：软件优化

| 方法 | 说明 | 效果 |
|------|------|------|
| 降低 num_ctx | 4096/8192 | 减少 KV Cache |
| 使用更低量化 | Q2_K / Q3_K | 减少权重 |
| 启用 Flash Attention | v0.11.8+ | 显存降低 15%，速度提升 16% |

#### 启用 Flash Attention

```bash
# Linux
export OLLAMA_FLASH_ATTENTION=1

# 或编辑 systemd 服务
sudo systemctl edit ollama
Environment="OLLAMA_FLASH_ATTENTION=1"
```

### 方案二：更换模型

对于单卡 24GB 显存环境，推荐：

| 模型 | 参数量 | 量化 | 显存需求 |
|------|--------|------|----------|
| DeepSeek-Coder-33B | 33B | Q4_K_M | ~20GB |
| CodeLlama-34B | 34B | Q4_K_M | ~20GB |
| Qwen2.5-Coder-14B | 14B | Q4_K_M | ~8GB |

### 方案三：硬件升级

- 添加第二张 RTX 3090，使用张量并行
- 或使用云端 API

## OpenCode 配置

### Ollama 运行模型

```bash
# 拉取模型
ollama pull qwen3-coder-next

# 测试运行
ollama run qwen3-coder-next
```

### OpenCode 配置

```json
{
  "$schema": "https://opencode.ai/config.json",
  "provider": {
    "ollama-qwen": {
      "npm": "@ai-sdk/ollama",
      "name": "qwen3-coder-next",
      "options": {
        "baseURL": "http://localhost:11434"
      },
      "models": {
        "qwen3-coder-next": {
          "name": "qwen3-coder-next"
        }
      }
    }
  }
}
```

> 💡 如果在 Docker 中运行 OpenCode，`localhost` 需改为 `host.docker.internal`

### 验证配置

保存配置后，重启 OpenCode，运行 `/models` 命令查看可用模型列表。

## 上下文长度优化

### 显存占用模型

- 模型权重：33B 参数，4-bit 量化后约 19-20 GB
- 可用显存：RTX 3090 实际可用约 23.5 GB
- KV cache：4 × 64 × 7168 ≈ 1.75 MB/token

### 不同方案对比

| 方案 | 上下文上限 | 说明 |
|------|------------|------|
| GPTQ/AWQ + float16 KV | ~2000 tokens | 实现简单 |
| GGUF + 8-bit KV | ~4000 tokens | 平衡方案 |
| GGUF + 4-bit KV | ~8000 tokens | **推荐** |
| GGUF + Q3 + 4-bit KV | ~12000 tokens | 最长上下文 |

### 推荐配置

```bash
./main -m deepseek-coder-33b-instruct.Q4_K_M.gguf \
       -ngl 999 \
       -c 8192 \
       --cache-type-q q4_0
```

## 量化格式对比

### GGUF 量化格式一览

| 量化格式 | 每权重建议 | 模型大小 | 7B 显存 | 33B 显存 | 推荐场景 |
|----------|-----------|----------|---------|----------|----------|
| Q4_K_M | ~4.5 bit | 30% FP16 | ~4.5GB | ~20GB | **通用推荐** |
| Q5_K_M | ~5.5 bit | 35% FP16 | ~5.5GB | ~25GB | 复杂推理 |
| Q8_0 | 8 bit | 50% FP16 | ~8GB | ~40GB | 极致质量 |

### 性能基准测试

| 任务类型 | FP16 | Q8_0 | Q5_K_M | Q4_K_M |
|----------|------|------|--------|--------|
| 代码生成 (HumanEval) | 51.2% | 50.8% | 50.1% | 49.5% |
| 推理速度 (tokens/s) | 35 | 42 | 48 | 55 |
| 复杂数学推理 | 100% | 98% | 95% | 92% |

### 选择建议

- **Q4_K_M**: 通用场景首选，质量和速度的最佳平衡，~2-3% 质量损失
- **Q5_K_M**: 复杂推理任务，有额外显存时升级
- **Q8_0**: 关键应用或基准测试，追求原生质量

### 常见误区

1. **迷信最高质量**: 大多数任务 Q4_K_M 和 Q8_0 质量差异几乎不可感知
2. **忽略上下文显存**: 16K tokens 上下文额外占用 ~4GB 显存
3. **使用旧格式**: 永远优先选择 K-quant 格式 (Q4_K_M > Q4_0)

## 总结

1. **模型选择**: 33B 级模型是单卡 24GB 的最优选择
2. **量化格式**: GGUF Q4_K_M 是最佳平衡点
3. **性能瓶颈**: 60% GPU 利用率通常是显存不足导致的带宽瓶颈
4. **优化优先级**: 降低 num_ctx > 启用 Flash Attention > 更换量化等级
5. **进阶方案**: 双卡并行或云端 API

掌握这些技巧，你可以在有限的硬件条件下充分发挥编程大模型的能力。
