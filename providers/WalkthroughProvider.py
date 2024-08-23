import sys
import argparse
from provider import *

def main():
    parser = argparse.ArgumentParser(
        prog="WalkthroughProvider",
        description="Fetches data from various walkthrough websites",
    )
    subparser = parser.add_subparsers()
    search_parser = subparser.add_parser('search', help='Search for available guides for a given title')
    search_parser.add_argument('title', type=str, help='Game title to use as the search query')
    args = parser.parse_args()

    provider = Provider(AvailableProviders.GAMEFAQS)
    results = provider.provider.search(args.title)
    print(provider.provider.get_guides(results[0]["url"]))

if __name__ == "__main__":
    main()