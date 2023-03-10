class TreeNode {
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
function newTreeNode (val, left, right) {
  return new TreeNode(val, left, right)
}

/**
  * @param {number} val
  * @param {TreeNode} left
  * @returns {TreeNode}
  */
function newTreeLeft (val, left) {
  return new TreeNode(val, left)
}

/**
  * @param {number} val
  * @param {TreeNode} right
  * @returns {TreeNode}
  */
function newTreeRight (val, right) {
  return new TreeNode(val, undefined, right)
}

/**
  * @param {number} val
  * @returns {TreeNode}
  */
function newTreeVal (val) {
  return new TreeNode(val)
}

class ListNode {
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
function newListNode (val, next) {
  return new ListNode(val, next)
}

/**
  * @param {number[]} vals
  * @returns {ListNode|null}
  */
function newList (vals) {
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
function newCycleList (vals, pos) {
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

module.exports = {
  TreeNode,
  newTreeNode,
  newTreeLeft,
  newTreeRight,
  newTreeVal,
  ListNode,
  newListNode,
  newList,
  newCycleList
}
