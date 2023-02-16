class TreeNode {
  /**
    * @param {number|undefined|null} val
    * @param {TreeNode|undefined|null} left
    * @param {TreeNode|undefined|null} right
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
function newnode (val, left, right) {
  return new TreeNode(val, left, right)
}

/**
  * @param {number} val
  * @param {TreeNode} right
  * @returns {TreeNode}
  */
function newright (val, right) {
  return new TreeNode(val, undefined, right)
}

/**
  * @param {number} val
  * @returns {TreeNode}
  */
function newval (val) {
  return new TreeNode(val)
}

/**
  * @param {TreeNode|null} node
  * @param {number} depth
  * @param {number[]} ref
  * @returns {undefined}
  */
function travel (node, depth, ref) {
  if (node != null) {
    const d = depth + 1
    travel(node.left, d, ref)
    travel(node.right, d, ref)
  } else {
    ref[0] = Math.max(ref[0], depth)
  }
}

/**
  * @param {TreeNode|null} node
  * @returns {number}
  */
function maxDepth (root) {
  const result = [0]
  travel(root, 0, result)

  return result[0]
}

async function main () {
  const inputs = [
    newnode(3, newval(9), newnode(20, newval(15), newval(7))),
    newright(1, newval(2))
  ]

  for (const root of inputs) {
    const result = maxDepth(root)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
