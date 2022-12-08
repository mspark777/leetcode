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

function newleft (val, left) {
  return new TreeNode(val, left)
}

function newright (val, right) {
  return new TreeNode(val, undefined, right)
}

function newval (val) {
  return new TreeNode(val)
}

/**
 * @param {number[]} stack
 * @param {TreeNode | null} node
 * @returns {undefined}
 */
function dfs (stack, node) {
  if (node == null) {
    return
  }

  const { val, left, right } = node
  if ((left == null) && (right == null)) {
    stack.push(val)
  }

  dfs(stack, left)
  dfs(stack, right)
}

/**
 * @param {TreeNode | null} root1
 * @param {TreeNode | null} root2
 * @returns {boolean}
 */
function leafSimilar (root1, root2) {
  const stack1 = []
  const stack2 = []
  dfs(stack1, root1)
  dfs(stack2, root2)

  if (stack1.length !== stack2.length) {
    return false
  }

  for (let i = 0; i < stack1.length; i += 1) {
    if (stack1[i] !== stack2[i]) {
      return false
    }
  }

  return true
}

async function main () {
  const inputs = [
    {
      root1: newnode(3,
        newnode(5,
          newval(6),
          newnode(2,
            newval(7),
            newval(4)
          )
        ),
        newnode(1,
          newval(9),
          newval(8)
        )
      ),
      root2: newnode(3,
        newnode(5, newval(6), newval(7)),
        newnode(1,
          newval(4),
          newnode(2,
            newval(9),
            newval(8)
          )
        )
      )
    },
    {
      root1: newnode(1, newval(2), newval(3)),
      root2: newnode(1, newval(3), newval(2))
    }
  ]

  for (const { root1, root2 } of inputs) {
    const result = leafSimilar(root1, root2)
    console.log(result)
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})
