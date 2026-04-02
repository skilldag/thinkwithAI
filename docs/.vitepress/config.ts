import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "My Blog",
  description: "A personal blog",
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
          { text: 'My First Post', link: '/posts/first-post' }
        ]
      }
    ]
  }
})
