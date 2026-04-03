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
        text: 'Posts',
        items: [
          { text: '我的 AI 实践笔记', link: '/posts/first-post' },
        {
          text: '本地编码大模型实践指南',
          link: '/posts/local-coding-llm-practice',
          collapsible: true,
          items: [
            { text: 'RTX 3090 编程大模型横评', link: '/posts/rtx3090-llm-benchmark' },
            { text: 'Qwen3-Coder-Next 在 RTX 3090 上的运行问题与优化指南', link: '/posts/qwen3-coder-next-rtx3090' }
          ]
        },
        {
          text: 'Harness',
          items: [
            { text: 'Harness Engineer', link: '/harness/harness-engineer' },
            { text: 'Claw Code：如何在24小时内获得10万Star', link: '/harness/claw-code-omx-100k-stars' }
          ]
        }
          },
        {
          text: 'Harness',
          items: [
            { text: 'Harness Engineer', link: '/harness/harness-engineer' },
            { text: 'Claw Code：如何在24小时内获得10万Star', link: '/harness/claw-code-omx-100k-stars' }
          ]
        },
      }
    ]
  }
})