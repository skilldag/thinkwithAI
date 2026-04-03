import asyncio
from pyppeteer import launch


async def fetch_deepseek_chat(url):
    browser = await launch(
        headless=True, args=["--no-sandbox", "--disable-setuid-sandbox"]
    )
    page = await browser.newPage()

    await page.goto(url, waitUntil="networkidle2")

    # Wait for chat content to load
    await asyncio.sleep(2)

    # Extract messages
    messages = await page.evaluate("""() => {
        const result = [];
        const msgElements = document.querySelectorAll('[class*="message"], [class*="chat-item"]');
        msgElements.forEach(el => {
            const role = el.querySelector('[class*="role"], [class*="avatar"]')?.textContent?.trim();
            const content = el.querySelector('[class*="content"], [class*="text"]')?.textContent?.trim();
            if (content) {
                result.push({ role, content });
            }
        });
        return result;
    }""")

    await browser.close()
    return messages


if __name__ == "__main__":
    url = "https://chat.deepseek.com/a/chat/s/7a9be694-b817-4654-90da-f690b7871d8b"
    messages = asyncio.get_event_loop().run_until_complete(fetch_deepseek_chat(url))
    for msg in messages:
        print(f"Role: {msg.get('role', 'unknown')}")
        print(f"Content: {msg.get('content', '')}")
        print("---")
