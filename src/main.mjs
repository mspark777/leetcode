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

function newright (val, right) {
  return newnode(val, null, right)
}

function newval (val) {
  return newnode(val)
}

/**
 * @param {TreeNode} root
 * @param {number} k
 * @returns {boolean}
 */
function findTarget (root, k) {
  const stack = [root]
  const memo = new Set()

  while (stack.length > 0) {
    const top = stack.pop()
    if (top == null) {
      continue
    }

    const { left, right, val } = top
    const target = k - val
    if (memo.has(target)) {
      return true
    }

    memo.add(val)
    stack.push(left, right)
  }

  return false
}

async function main () {
  const inputs = [
    {
      root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
      k: 9
    },
    {
      root: newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))),
      k: 28
    }
  ]

  for (const { root, k } of inputs) {
    const result = findTarget(root, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
