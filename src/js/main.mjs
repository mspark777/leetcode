class TrieNode {
  /** @type {TrieNode} */
  #links
  /** @type {boolean} */
  #ended
  constructor () {
    this.#links = new Array(26)
    this.#ended = false
  }

  /**
    * @param {string} ch
    * @returns {boolean}
    */
  containsKey (ch) {
    return this.get(ch) != null
  }

  /**
    * @param {string} ch
    * @returns {TrieNode|undefined}
    */
  get (ch) {
    const i = this.#getIndex(ch)
    return this.#links.at(i)
  }

  /**
    * @param {string} ch
    * @param {TrieNode} node
    * @return {undefined}
    */
  put (ch, node) {
    const i = this.#getIndex(ch)
    this.#links[i] = node
  }

  /**
    * @returns {undefined}
    */
  setEnd () {
    this.#ended = true
  }

  /**
    * @returns {boolean}
    */
  isEnd () {
    return this.#ended
  }

  /**
    * @param {string} ch
    * @returns {number}
    */
  #getIndex (ch) {
    const code = ch.charCodeAt(0)
    const acode = 'a'.charCodeAt(0)
    return code - acode
  }
}

class Trie {
  /** @type {TrieNode} */
  #root
  constructor () {
    this.#root = new TrieNode()
  }

  /**
    * @param {string} word
    * @returns {undefined}
    */
  insert (word) {
    let node = this.#root
    for (const ch of word) {
      if (!node.containsKey(ch)) {
        node.put(ch, new TrieNode())
      }

      node = node.get(ch)
    }

    node.setEnd()
  }

  /**
    * @param {string} word
    * @returns {boolean}
    */
  search (word) {
    const node = this.#searchPrefix(word)
    return node?.isEnd() === true
  }

  /**
    * @param {string} prefix
    * @returns {boolean}
    */
  startsWith (prefix) {
    const node = this.#searchPrefix(prefix)
    return node != null
  }

  /**
    * @param {string} prefix
    * @returns {TrieNode|undefined}
    */
  #searchPrefix (word) {
    let node = this.#root
    for (const ch of word) {
      node = node.get(ch)
      if (node == null) {
        return
      }
    }

    return node
  }
}

async function main () {
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
