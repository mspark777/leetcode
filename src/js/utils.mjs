export function unused (..._args) {}

export class TreeNode {
  /** @type {number} */
  val
  /** @type {TreeNode | null} */
  left
  /** @type {TreeNode | null} */
  right

  /**
    * @param {number | undefined} val
    * @param {TreeNode | null | undefined} left
    * @param {TreeNode | null | undefined} right
    */
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

/**
  * @param {number} val
  * @param {TreeNode} left
  * @param {TreeNode} right
  * @returns {TreeNode}
  */
export function newTreeNode (val, left, right) {
  return new TreeNode(val, left, right)
}

/**
  * @param {number} val
  * @param {TreeNode} left
  * @returns {TreeNode}
  */
export function newTreeLeft (val, left) {
  return new TreeNode(val, left)
}

/**
  * @param {number} val
  * @param {TreeNode} right
  * @returns {TreeNode}
  */
export function newTreeRight (val, right) {
  return new TreeNode(val, undefined, right)
}

/**
  * @param {number} val
  * @returns {TreeNode}
  */
export function newTreeVal (val) {
  return new TreeNode(val)
}

export class ListNode {
  /** @type {number} */
  val
  /** @type {ListNode|null} */
  next

  /**
    * @param {number|undefined} val
    * @param {ListNode|undefined|null} next
    */
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

/**
  * @param {number} val
  * @param {ListNode|undefined} next
  * @returns {ListNode}
  */
export function newListNode (val, next) {
  return new ListNode(val, next)
}

/**
  * @param {number[]} vals
  * @returns {ListNode|null}
  */
export function newList (vals) {
  const head = new ListNode()
  let tail = head
  for (const val of vals) {
    const node = newListNode(val)
    tail.next = node
    tail = node
  }

  return head.next
}

/**
  * @param {number[]} vals
  * @param {number} pos
  * @returns {ListNode|null}
  */
export function newCycleList (vals, pos) {
  const head = new ListNode()
  let tail = head
  let target = null
  for (let i = 0; i < vals.length; i += 1) {
    const node = newListNode(vals[i])
    if (i === pos) {
      target = node
    }

    tail.next = node
    tail = node
  }
  tail.next = target

  return head.next
}

export class TrieNode {
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

export class Trie {
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

export class UnionFind {
  /** @type {number[]} */
  #parents
  /** @type {number[]} */
  #ranks

  /**
    * @param {number} size
    */
  constructor (size) {
    this.#parents = Array.from(new Array(size), (_, i) => i)
    this.#ranks = new Array(size).fill(0)
  }

  /**
    * @param {number} x
    * @returns {number}
    */
  find (x) {
    if (this.#parents[x] !== x) {
      this.#parents[x] = this.find(this.#parents[x])
    }
    return this.#parents[x]
  }

  /**
    * @param {number} x
    * @param {number} y
    * @returns {undefined}
    */
  union (x, y) {
    const xset = this.find(x)
    const yset = this.find(y)
    if (xset === yset) {
      return
    }

    if (this.#ranks[xset] < this.#ranks[yset]) {
      this.#parents[xset] = yset
    } else if (this.#ranks[xset] > this.#ranks[yset]) {
      this.#parents[yset] = xset
    } else {
      this.#parents[yset] = xset
      this.#ranks[xset] += 1
    }
  }
}
