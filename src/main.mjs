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
  * @param {TreeNode} left
  * @returns {TreeNode}
  */
function newleft (val, left) {
  return new TreeNode(val, left)
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
  * @param {TreeNode|null} root
  * @param {{prevNode: TreeNode | undefined, minDistance: number}} data
  * @returns {number}
  */
function travel (root, data) {
  if (root == null) {
    return data.minDistance
  }

  travel(root.left, data)

  if (data.prevNode != null) {
    data.minDistance = Math.min(data.minDistance, root.val - data.prevNode.val)
  }
  data.prevNode = root
  travel(root.right, data)
  return data.minDistance
}

/**
  * @param {TreeNode|null} root
  * @returns {number}
  */
function minDiffInBST (root) {
  return travel(root, { minDistance: Number.MAX_SAFE_INTEGER })
}

async function main () {
  const inputs = [
    newnode(4, newnode(2, newval(1), newval(3)), newval(6)),
    newnode(1, newval(0), newnode(48, newval(12), newval(49)))
  ]

  for (const root of inputs) {
    const result = minDiffInBST(root)
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
