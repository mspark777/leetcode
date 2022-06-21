class TrieNode {
  constructor () {
    this.isWord = false
    this.children = new Array(26)
  }
}

class Trie {
  constructor () {
    this.root = new TrieNode()
    this.chars = 'abcdefghijklmnopqrstuvwxyz'
  }

  insert (str) {
    let cur = this.root
    const acode = this.chars.charCodeAt(0)
    for (let i = 0; i < str.length; i += 1) {
      const code = str.charCodeAt(i)
      const j = code - acode
      if (!cur.children[j]) {
        cur.children[j] = new TrieNode()
      }

      cur = cur.children[j]
    }
    cur.isWord = true
  }

  getWords (prefix) {
    let cur = this.root
    const result = []

    const acode = this.chars.charCodeAt(0)
    for (let i = 0; i < prefix.length; i += 1) {
      const code = prefix.charCodeAt(i)
      const j = code - acode
      if (cur.children[j]) {
        cur = cur.children[j]
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

    const acode = this.chars.charCodeAt(0)
    const chars = this.chars
    for (let i = 0; i < chars.length; i += 1) {
      const code = chars.charCodeAt(i)
      const j = code - acode
      const child = cur.children[j]
      if (child) {
        this.dfs(child, `${word}${chars.charAt(i)}`, result)
      }
    }
  }
}

export function suggestedProducts1 (products, searchWord) {
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
