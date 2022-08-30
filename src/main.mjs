class TreeNode {
  constructor (val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function binaryTreePaths (root) {
  if (root == null) {
    return []
  }

  const stack = [{
    path: [],
    node: root
  }]

  const result = []
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { node, path } = top
    path.push(node)

    if ((node.left == null) && (node.right == null)) {
      result.push(path.map(p => p.val))
      continue
    }

    if (node.left != null) {
      stack.push({
        path: [...path],
        node: node.left
      })
    }

    if (node.right != null) {
      stack.push({
        path: [...path],
        node: node.right
      })
    }
  }

  return result.map(r => r.join('->'))
}

async function main () {
  const inputs = [
    {
      root: new TreeNode(1,
        new TreeNode(2,
          null,
          new TreeNode(5)
        ),
        new TreeNode(3)
      )
    },
    {
      root: new TreeNode(1)
    }
  ]

  for (const { root } of inputs) {
    const result = binaryTreePaths(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
