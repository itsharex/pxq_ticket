import asyncio
import os

import requests
from telegram import Bot


def download_latest_release_file(owner, repo, token):
    # Get the latest release information
    headers = {
        'User-Agent': 'Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) '
                      'Chrome/122.0.0.0 Mobile Safari/537.36',
        'Authorization': 'Bearer {}'.format(token)
    }
    release_url = f"https://api.github.com/repos/{owner}/{repo}/releases/latest"
    response = requests.get(release_url, headers=headers)
    release_info = response.json()
    asset_name_list = []
    for item in release_info['assets']:
        asset_name = item['name']
        if asset_name.endswith('.AppImage'):
            continue
        asset_url = item['browser_download_url']
        response = requests.get(asset_url)
        with open(asset_name, 'wb') as f:
            f.write(response.content)
            asset_name_list.append(asset_name)
    return asset_name_list, release_info['body']


async def upload_to_telegram(bot_token, chat_id, owner, repo, token):
    bot = Bot(token=bot_token)
    file_paths, message = download_latest_release_file(owner, repo, token)
    msg = await bot.send_message(chat_id=chat_id, text=message, parse_mode='HTML')
    for file_path in file_paths:
        with open(file_path, 'rb') as f:
            await bot.send_document(chat_id=chat_id, document=f, message_thread_id=msg.message_thread_id)


def main():
    bot_token = os.environ.get('TELEGRAM_BOT_TOKEN')
    chat_id = os.environ.get('TELEGRAM_CHAT_ID')
    owner = os.environ.get('OWNER')
    repo = os.environ.get('REPO')
    token = os.environ.get('ACCESS_TOKEN')
    asyncio.run(upload_to_telegram(bot_token, chat_id, owner, repo, token))


if __name__ == "__main__":
    main()
