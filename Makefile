.PHONY: install build deploy clean

install:
	pnpm install
	mkdir -p ~/.agents/skills
	cp -r ai-blog-generator ~/.agents/skills/

build:
	pnpm build

deploy:
	cp -r docs/.vitepress/dist/* docs/
	git add -A
	git commit -m "feat: 更新博客" || true
	git push origin gh-pages

clean:
	rm -rf docs/.vitepress/dist/
