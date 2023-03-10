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
