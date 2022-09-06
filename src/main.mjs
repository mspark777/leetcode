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

function preorder (node) {
  if (node != null) {
    preorder(node.left)
    process.stdout.write(`${node.val} `)
    preorder(node.right)
  }
}

/**
 * @param {TreeNode | null} root
 * @returns {boolean}
 */
function containsOne (node) {
  if (node == null) {
    return false
  }

  const leftContained = containsOne(node.left)
  if (!leftContained) {
    node.left = null
  }

  const rightContained = containsOne(node.right)
  if (!rightContained) {
    node.right = null
  }

  return node.val === 1 || leftContained || rightContained
}

/**
 * @param {TreeNode} root
 * @returns {TreeNode}
 */
function pruneTree (root) {
  return containsOne(root) ? root : null
}

async function main () {
  const inputs = [
    {
      root: newright(1,
        newnode(0,
          newval(0),
          newval(1)
        )
      )
    },
    {
      root: newnode(1,
        newnode(0,
          newval(0),
          newval(0)
        ),
        newnode(1,
          newval(0),
          newval(1)
        )
      )
    },
    {
      root: newnode(1,
        newnode(1,
          newleft(1,
            newval(0)
          ),
          newval(1)
        ),
        newnode(0,
          newval(0),
          newval(1)
        )
      )
    }
  ]

  for (const { root } of inputs) {
    const result = pruneTree(root)
    preorder(result)
    console.log()
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
