export class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function newTreeNode (val: number, left: TreeNode, right: TreeNode): TreeNode {
  return new TreeNode(val, left, right)
}

export function newTreeLeft (val: number, left: TreeNode): TreeNode {
  return new TreeNode(val, left)
}

export function newTreeRight (val: number, right: TreeNode): TreeNode {
  return new TreeNode(val, undefined, right)
}

export function newTreeVal (val: number): TreeNode {
  return new TreeNode(val)
}

export class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

export function newListNode (val: number, next?: ListNode): ListNode {
  return new ListNode(val, next)
}

export function newList (vals: number[]): ListNode | null {
  const head = new ListNode()
  let tail = head
  for (const val of vals) {
    const node = newListNode(val)
    tail.next = node
    tail = node
  }

  return head.next
}

export function newCycleList (vals: number[], pos: number): ListNode | null {
  const head = new ListNode()
  let tail = head
  let target: ListNode | null = null
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

export class Trie {
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

export class UnionFind {
  private readonly parents: number[]
  private readonly ranks: number[]
  public constructor (size: number) {
    this.parents = Array.from(new Array(size), (_, i) => i)
    this.ranks = new Array(size).fill(0)
  }

  public find (x: number): number {
    if (this.parents[x] !== x) {
      this.parents[x] = this.find(this.parents[x])
    }
    return this.parents[x]
  }

  public union (x: number, y: number): void {
    const xset = this.find(x)
    const yset = this.find(y)
    if (xset === yset) {
      return
    }

    if (this.ranks[xset] < this.ranks[yset]) {
      this.parents[xset] = yset
    } else if (this.ranks[xset] > this.ranks[yset]) {
      this.parents[yset] = xset
    } else {
      this.parents[yset] = xset
      this.ranks[xset] += 1
    }
  }
}
