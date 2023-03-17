from typing import Optional

from utils import Trie


def main():
    trie = Trie()
    trie.insert("apple")
    print(trie.search("apple"))
    print(trie.search("app"))
    print(trie.startsWith("app"))
    trie.insert("app")
    print(trie.search("app"))


if __name__ == "__main__":
    main()
