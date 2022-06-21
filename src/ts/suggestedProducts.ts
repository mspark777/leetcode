namespace Version0 {
  export class TrieNode {
    isWord: boolean
    // eslint-disable-next-line no-use-before-define
    children: Map<number, TrieNode>
    constructor () {
      this.isWord = false
      this.children = new Map()
    }
  }

  export class Trie {
    root: TrieNode
    chars: string
    constructor () {
      this.root = new TrieNode()
      this.chars = 'abcdefghijklmnopqrstuvwxyz'
    }

    insert (str: string): void {
      let cur = this.root
      for (let i = 0; i < str.length; i += 1) {
        const code = str.charCodeAt(i)
        if (!cur.children.has(code)) {
          cur.children.set(code, new TrieNode())
        }
        cur = cur.children.get(code) as TrieNode
      }
      cur.isWord = true
    }

    getWords (prefix: string): string[] {
      let cur = this.root
      const result: string[] = []

      for (let i = 0; i < prefix.length; i += 1) {
        const code = prefix.charCodeAt(i)
        if (cur.children.has(code)) {
          cur = cur.children.get(code) as TrieNode
        } else {
          return result
        }
      }

      this.dfs(cur, prefix, result)
      return result
    }

    dfs (cur: TrieNode, word: string, result: string[]): void {
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
}

export function suggestedProducts (products: string[], searchWord: string): string[][] {
  const trie = new Version0.Trie()
  for (const product of products) {
    trie.insert(product)
  }

  const result: string[][] = []
  for (let i = 1; i <= searchWord.length; i += 1) {
    const sub = searchWord.substring(0, i)
    const words = trie.getWords(sub)
    result.push(words)
  }

  return result
}

namespace Version1 {
  export class TrieNode {
    isWord: boolean
    // eslint-disable-next-line no-use-before-define
    children: TrieNode[]
    constructor () {
      this.isWord = false
      this.children = new Array(26)
    }
  }

  export class Trie {
    root: TrieNode
    chars: string
    constructor () {
      this.root = new TrieNode()
      this.chars = 'abcdefghijklmnopqrstuvwxyz'
    }

    insert (str: string): void {
      let cur = this.root
      const acode = this.chars.charCodeAt(0)
      for (let i = 0; i < str.length; i += 1) {
        const code = str.charCodeAt(i)
        const j = code - acode
        if (!cur.children[j]) {
          cur.children[j] = new TrieNode()
        }

        cur = cur.children[j] as TrieNode
      }
      cur.isWord = true
    }

    getWords (prefix: string): string[] {
      let cur = this.root
      const result: string[] = []

      const acode = this.chars.charCodeAt(0)
      for (let i = 0; i < prefix.length; i += 1) {
        const code = prefix.charCodeAt(i)
        const j = code - acode
        if (cur.children[j]) {
          cur = cur.children[j] as TrieNode
        } else {
          return result
        }
      }

      this.dfs(cur, prefix, result)
      return result
    }

    dfs (cur: TrieNode, word: string, result: string[]): void {
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
}

export function suggestedProducts1 (products: string[], searchWord: string): string[][] {
  const trie = new Version1.Trie()
  for (const product of products) {
    trie.insert(product)
  }

  const result: string[][] = []
  for (let i = 1; i <= searchWord.length; i += 1) {
    const sub = searchWord.substring(0, i)
    const words = trie.getWords(sub)
    result.push(words)
  }

  return result
}

export function suggestedProducts2 (products: string[], searchWord: string): string[][] {
  products.sort()

  const results: string[][] = []
  for (let i = 1; i <= searchWord.length; i += 1) {
    const sub = searchWord.substring(0, i)
    const result: string[] = []
    for (const product of products) {
      if (result.length >= 3) {
        break
      } else if (product.startsWith(sub)) {
        result.push(product)
      }
    }
    results.push(result)
  }

  return results
}

export function suggestedProducts3 (products: string[], searchWord: string): string[][] {
  products.sort()

  const results: string[][] = []
  for (let i = 0; i < searchWord.length; i += 1) {
    products = products.filter(p => p[i] === searchWord[i])
    results.push(products.slice(0, 3))
  }

  return results
}

export function suggestedProducts4 (products: string[], searchWord: string): string[][] {
  products.sort()

  const results: string[][] = new Array(searchWord.length)
  for (let i = 0; i < searchWord.length; i += 1) {
    results[i] = []
  }

  for (const product of products) {
    const len = Math.min(product.length, searchWord.length)
    for (let i = 0; i < len; i += 1) {
      if (product.charCodeAt(i) !== searchWord.charCodeAt(i)) {
        break
      } else if (results[i].length < 3) {
        results[i].push(product)
      }
    }
  }

  return results
}
