import re
import requests
import urllib.parse
import json
from fake_useragent import UserAgent
from bs4 import BeautifulSoup, NavigableString

class Gamefaqs():
    def __init__(self):
        self.header = {"User-Agent": UserAgent().random}
        self.baseurl = "https://gamefaqs.gamespot.com"
        self.url = self.baseurl
        self.search_url = "/ajax/home_game_search?term=&term="

    def search(self, title: str):
        url = self.baseurl + self.search_url + urllib.parse.quote(title)
        results = json.loads(requests.get(url, headers=self.header).text)

        return results

    def get_guides(self, url: str) -> str:
        page = requests.get(self.baseurl + url + "/faqs", headers=self.header)
        soup = BeautifulSoup(page.text, 'html.parser')

        guides_dict = {"guides": []}
        guides_lists = soup.find_all("ol", "guides")
        for guides_list in guides_lists:
            for ol in guides_list:
                if type(ol) is NavigableString:
                    continue
                info = {}
                a = ol.find("a")
                info["link"] = a["href"]
                info["title"] = a.find_all(string=True, recursive=False)
                guides_dict["guides"].append(info)
        
        return json.dumps(guides_dict)

        # print(guides)

        # with open("out.html", "w+") as f:
        #     f.write((str)())

        # print(soup.find_all("ol", "guides"))
