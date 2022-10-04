class TreeNode {
  val
  left
  right
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function newnode (val, left, right) {
  return new TreeNode(val, left, right)
}

function newleft (val, left) {
  return newnode(val, left)
}

function newright (val, right) {
  return newnode(val, undefined, right)
}

function newval (val) {
  return newnode(val)
}

/**
 * @param {TreeNode} root
 * @param {number} targetSum
 * @returns {boolean}
 */
function hasPathSum (root, targetSum) {
  if (root == null) {
    return false
  }

  const stack = [{ node: root, target: targetSum }]
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { node, target } = top
    const { val, left, right } = node
    const newTarget = target - val
    let isleaf = true
    if (left != null) {
      isleaf = false
      stack.push({
        node: left,
        target: newTarget
      })
    }

    if (right != null) {
      isleaf = false
      stack.push({
        node: right,
        target: newTarget
      })
    }

    if (isleaf && (newTarget === 0)) {
      return true
    }
  }

  return false
}

async function main () {
  const inputs = [
    {
      root: newnode(5, newleft(4, newnode(11, newval(7), newval(2))), newnode(8, newval(13), newright(4, newval(1)))),
      targetSum: 22
    },
    {
      root: newnode(5, newval(2), newval(3)),
      targetSum: 5
    },
    {
      root: null,
      targetSum: 0
    }
  ]

  for (const { root, targetSum } of inputs) {
    const result = hasPathSum(root, targetSum)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
