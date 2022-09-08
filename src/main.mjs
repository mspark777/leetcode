class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function newnode (val, left, right) {
  return new TreeNode(val, left, right)
}

function newval (val) {
  return newnode(val, null, null)
}

function newleft (val, left) {
  return newnode(val, left, null)
}

function newright (val, right) {
  return newnode(val, null, right)
}

/**
 * @param {TreeNode} root
 * @return {number[]}
 */
function inorderTraversal (root) {
  const result = []
  const travel = node => {
    if (node != null) {
      travel(node.left)
      result.push(node.val)
      travel(node.right)
    }
  }
  travel(root)

  return result
}

async function main () {
  const inputs = [
    {
      root: newright(1, newleft(2, newval(3)))
    },
    {
      root: null
    },
    {
      root: newval(1)
    }
  ]

  for (const { root } of inputs) {
    const result = inorderTraversal(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
