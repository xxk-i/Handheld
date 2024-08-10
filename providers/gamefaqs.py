import re
import requests
import urllib.parse
import sys
import json
from fake_useragent import UserAgent
from bs4 import BeautifulSoup

gamefaqs = "https://gamefaqs.gamespot.com"
search = "/ajax/home_game_search?term=&term="

def main(game_title: str):
    header = {"User-Agent": UserAgent().random}

    url = gamefaqs + search + urllib.parse.quote(game_title)

    results = json.loads(requests.get(url, headers=header).text)

    page = requests.get(gamefaqs + results[0]["url"] + "/faqs", headers=header)

    soup = BeautifulSoup(page.text, 'html.parser')

    print(soup.find_all("ol", "guides"))

if __name__ == "__main__":
    main(sys.argv[1])