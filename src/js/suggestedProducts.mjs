class TrieNode {
  constructor () {
    this.isWord = false
    this.children = new Map()
  }
}

class Trie {
  constructor () {
    this.root = new TrieNode()
    this.chars = 'abcdefghijklmnopqrstuvwxyz'
  }

  insert (str) {
    let cur = this.root
    for (let i = 0; i < str.length; i += 1) {
      const code = str.charCodeAt(i)
      if (!cur.children.has(code)) {
        cur.children.set(code, new TrieNode())
      }
      cur = cur.children.get(code)
    }
    cur.isWord = true
  }

  getWords (prefix) {
    let cur = this.root
    const result = []

    for (let i = 0; i < prefix.length; i += 1) {
      const code = prefix.charCodeAt(i)
      if (cur.children.has(code)) {
        cur = cur.children.get(code)
      } else {
        return result
      }
    }

    this.dfs(cur, prefix, result)
    return result
  }

  dfs (cur, word, result) {
    if (result.length === 3) {
      return
    } else if (cur.isWord) {
      result.push(word)
    }

    const chars = this.chars
    for (let i = 0; i < chars.length; i += 1) {
      const code = chars.charCodeAt(i)
      const child = cur.children.get(code)
      if (child) {
        this.dfs(child, `${word}${chars.charAt(i)}`, result)
      }
    }
  }
}

/**
 * @param {string[]} products
 * @param {string} searchWord
 * @return {string[][]}
 */
export function suggestedProducts (products, searchWord) {
  const trie = new Trie()
  for (const product of products) {
    trie.insert(product)
  }

  const result = []
  for (let i = 1; i <= searchWord.length; i += 1) {
    const sub = searchWord.substring(0, i)
    const words = trie.getWords(sub)
    result.push(words)
  }

  return result
}
