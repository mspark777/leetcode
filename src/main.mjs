class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
function invertTree (root) {
  const stack = [root]
  while (stack.length > 0) {
    const node = stack.pop()
    if (node == null) {
      continue
    }

    const left = node.left
    const right = node.right
    node.right = left
    node.left = right
    stack.push(left)
    stack.push(right)
  }

  return root
}

function travelInorder (node) {
  if (node != null) {
    travelInorder(node.left)
    process.stdout.write(`${node.val} `)
    travelInorder(node.right)
  }
}

async function main () {
  const inputs = [
    {
      root: new TreeNode(4,
        new TreeNode(2,
          new TreeNode(1),
          new TreeNode(3)
        ),
        new TreeNode(7,
          new TreeNode(6),
          new TreeNode(9)
        )
      )
    },
    {
      root: new TreeNode(2, new TreeNode(1), new TreeNode(3))
    },
    {
      root: null
    }
  ]

  for (const input of inputs) {
    const root = input.root
    const result = invertTree(root)
    travelInorder(result)
    console.log()
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
