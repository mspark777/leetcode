class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

/**
 * @param {TreeNode} root
 * @return {boolean}
 */
function isValidBST (root) {
  if (!root) {
    return true
  }

  let pre = null
  const stack = []
  while (root || (stack.length > 0)) {
    while (root) {
      stack.push(root)
      root = root.left
    }

    root = stack.pop()
    if (pre && (root.val <= pre.val)) {
      return false
    }
    pre = root
    root = root.right
  }

  return true
}

async function main () {
  const inputs = [
    {
      root: new TreeNode(2,
        new TreeNode(1),
        new TreeNode(3)
      )
    },
    {
      root: new TreeNode(4,
        new TreeNode(1),
        new TreeNode(4,
          new TreeNode(3),
          new TreeNode(6)
        )
      )
    }
  ]

  for (const { root } of inputs) {
    const result = isValidBST(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
