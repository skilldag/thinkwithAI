# OpenCode 存储目录结构详解

深入解析 OpenCode 在本地的存储目录结构和各个目录的作用。

## 目录概览

```
~/.local/share/opencode/
├── bin           # 58M - OpenCode 依赖的二进制工具
├── log           # 2.3M - 程序运行日志
├── opencode.db   # 736M - SQLite 数据库，存储会话、消息等
├── opencode.db-shm   # 32K - SQLite 共享内存
├── opencode.db-wal   # 2.0M - SQLite 预写日志
├── snapshot      # 1.5G - 代码快照库
├── storage      # 144M - 附件、嵌入向量等
└── tool-output  # 9.4M - 工具命令输出缓存
```

## 核心存储

### opencode.db (SQLite)

存储所有结构化数据：
- 会话记录
- 消息内容
- 用户设置

可以用 sqlite3 查看：

```bash
sqlite3 ~/.local/share/opencode/opencode.db
.tables
```

### snapshot/ - 代码快照

存储项目代码的历史快照，用于让 AI 理解代码变更：

```
snapshot/
  147baf8.../           # 项目标识
    511912f.../        # 快照 ID（类似 Git 仓库）
      HEAD, objects/, refs/
```

### storage/ - 附件存储

存储上传文件、嵌入向量等非结构化数据。

### tool-output/ - 缓存

工具命令输出缓存（如 ls、git diff 结果）。

## 会话数据位置

| 数据类型 | 存储位置 |
|----------|----------|
| 对话记录 | opencode.db |
| 代码快照 | snapshot/ |
| 附件文件 | storage/ |
| 工具输出 | tool-output/ |

## 注意事项

- snapshot 目录占用较大（1.5GB），可定期清理
- opencode.db 是核心数据，可备份
