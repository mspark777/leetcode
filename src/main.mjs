class Node {
  constructor (val) {
    this.val = (val === undefined ? 0 : val)
    this.children = []
  }
}

/**
 * @param {Node|null} root
 * @return {number[][]}
 */
function levelOrder (root) {
  if (root == null) {
    return []
  }

  const queue = [root]
  const result = []
  while (queue.length > 0) {
    const size = queue.length
    const values = new Array(size)
    for (let i = 0; i < size; i += 1) {
      const node = queue[i]
      values[i] = node.val
      queue.push(...node.children)
    }
    queue.splice(0, size)

    result.push(values)
  }

  return result
}

function newnode (val, ...children) {
  const node = new Node(val)
  node.children = children
  return node
}

async function main () {
  const inputs = [
    {
      root: newnode(1,
        newnode(3,
          newnode(5),
          newnode(6)
        ),
        newnode(2),
        newnode(4)
      )
    },
    {
      root: newnode(1, newnode(2), newnode(3, newnode(6), newnode(7, newnode(11, newnode(14)))), newnode(4, newnode(8, newnode(12))), newnode(5, newnode(9, newnode(13)), newnode(10)))
    }
  ]

  for (const { root } of inputs) {
    const result = levelOrder(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
