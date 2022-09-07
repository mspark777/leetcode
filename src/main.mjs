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
 * @return {string}
 */
function tree2str (root) {
  if (root == null) {
    return ''
  }

  const stack = [root]
  const visiteds = new Set()
  const result = []
  for (let node = stack.at(-1); node != null; node = stack.at(-1)) {
    if (visiteds.has(node)) {
      stack.pop()
      result.push(')')
      continue
    }

    visiteds.add(node)
    const { left, right, val } = node
    result.push('(', val.toString())
    if ((left == null) && (right != null)) {
      result.push('()')
    }

    if (right != null) {
      stack.push(right)
    }

    if (left != null) {
      stack.push(left)
    }
  }

  return result.slice(1, -1).join('')
}

async function main () {
  const inputs = [
    {
      root: newnode(1,
        newleft(2, newval(4)),
        newval(3)
      )
    },
    {
      root: newnode(1,
        newright(2, newval(4)),
        newval(3)
      )
    }
  ]

  for (const { root } of inputs) {
    const result = tree2str(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
