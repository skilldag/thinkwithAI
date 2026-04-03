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
            text: 'AI 生成博客工作流', 
            link: '/posts/ai-generated-blog-workflow',
            items: [
              { text: 'AI Blog Generator Skill 设计思路', link: '/posts/ai-blog-generator' },
              { text: 'DeepSeek Research Skill 设计思路', link: '/posts/deepseek-research' }
            ]
          },
          { text: 'OpenCode 全面指南', link: '/posts/opencode-guide' },
          { text: 'OpenCode 存储结构', link: '/posts/opencode-storage-structure' },
          { 
            text: '本地大模型', 
            link: '/posts/local-coding-llm-practice',
            items: [
              { text: 'Qwen3 Coder Next + RTX 3090', link: '/posts/qwen3-coder-next-rtx3090' },
              { text: 'RTX 3090 + DeepSeek Coder 33B', link: '/posts/rtx3090-deepseek-coder-33b' },
              { text: 'Chrome DevTools Protocol 自动化', link: '/posts/chrome-devtools-protocol' }
            ]
          }
        ]
      }
    ]
  }
})
