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
            collapsible: true,
            items: [
              { text: 'AI Blog Generator Skill 设计思路', link: '/posts/ai-blog-generator' },
              { text: 'DeepSeek Research Skill 设计思路', link: '/posts/deepseek-research' },
              { text: '使用 CDP 对 DeepSeek 网页对话数据建档', link: '/posts/chrome-devtools-protocol' },

            ]
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