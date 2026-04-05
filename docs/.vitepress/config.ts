import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "AI 实践笔记",
  description: "记录每天使用 AI 的想法和实践",
  base: '/thinkwithAI/',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Posts', link: '/posts/' }
    ],
    sidebar: [
      {
        text: '博客工作流',
        link: '/posts/ai-generated-blog-workflow',
        collapsible: true,
        items: [
          { text: 'AI 生成博客工作流', link: '/posts/ai-generated-blog-workflow' },
          { text: 'AI Blog Generator Skill 设计思路', link: '/posts/ai-blog-generator' },
          { text: 'DeepSeek Research Skill 设计思路', link: '/posts/deepseek-research' },
          { text: 'Chrome CDP 数据建档', link: '/posts/chrome-devtools-protocol' },
          { text: 'Chrome CDP 与 BM25', link: '/posts/chrome-cdp-bm25' }
        ]
      },
      {
        text: '本地编码大模型',
        link: '/posts/local-coding-llm-practice',
        collapsible: true,
        items: [
          { text: '本地编码大模型指南', link: '/posts/local-coding-llm-practice' },
          { text: '编程大模型优化方案', link: '/posts/local-coding-llm-optimization' },
          { text: 'RTX 3090 编程大模型横评', link: '/posts/rtx3090-llm-benchmark' },
          { text: 'Qwen3-Coder-Next RTX 3090 优化指南', link: '/posts/qwen3-coder-next-rtx3090' }
        ]
      },
      {
        text: 'OpenCode',
        link: '/posts/opencode-guide',
        collapsible: true,
        items: [
          { text: 'OpenCode 使用指南', link: '/posts/opencode-guide' },
          { text: 'OpenCode Sessions CLI', link: '/posts/opencode-sessions' },
          { text: 'OpenCode 存储结构', link: '/posts/opencode-storage-structure' }
        ]
      },
      {
        text: '研发思考',
        items: [
          { text: '产品引导研发', link: '/posts/product-driven-rd' }
        ]
      },
      {
        text: 'Harness',
        link: '/harness/harness-engineer',
        collapsible: true,
        items: [
          { text: 'Harness Engineer', link: '/harness/harness-engineer' },
          { text: '熵管理实现分析', link: '/harness/entropy-management-implementation' },
          { text: 'Claw Code：如何在24小时内获得10万Star', link: '/harness/claw-code-omx-100k-stars' }
        ]
      }
    ]
  }
})