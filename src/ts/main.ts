class TrieNode {
  private readonly links: TrieNode[]
  private ended: boolean
  public constructor () {
    this.links = new Array<TrieNode>(26)
    this.ended = false
  }

  public containsKey (ch: string): boolean {
    return this.get(ch) != null
  }

  public get (ch: string): TrieNode | undefined {
    const i = this.getIndex(ch)
    return this.links.at(i)
  }

  public put (ch: string, node: TrieNode): void {
    const i = this.getIndex(ch)
    this.links[i] = node
  }

  public setEnd (): void {
    this.ended = true
  }

  public isEnd (): boolean {
    return this.ended
  }

  private getIndex (ch: string): number {
    const code = ch.charCodeAt(0)
    const acode = 'a'.charCodeAt(0)
    return code - acode
  }
}

class Trie {
  private readonly root: TrieNode
  public constructor () {
    this.root = new TrieNode()
  }

  public insert (word: string): void {
    let node = this.root
    for (const ch of word) {
      if (!node.containsKey(ch)) {
        node.put(ch, new TrieNode())
      }

      node = node.get(ch) as TrieNode
    }

    node.setEnd()
  }

  public search (word: string): boolean {
    const node = this.searchPrefix(word)
    return node?.isEnd() === true
  }

  public startsWith (prefix: string): boolean {
    const node = this.searchPrefix(prefix)
    return node != null
  }

  private searchPrefix (word: string): TrieNode | undefined {
    let node: TrieNode | undefined = this.root
    for (const ch of word) {
      node = node.get(ch)
      if (node == null) {
        return
      }
    }

    return node
  }
}

async function main (): Promise<void> {
  const trie = new Trie()
  trie.insert('apple')
  console.log(trie.search('apple'))
  console.log(trie.search('app'))
  console.log(trie.startsWith('app'))
  trie.insert('app')
  console.log(trie.search('app'))
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
