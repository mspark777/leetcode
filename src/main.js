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
  return newnode(val, left, null)
}

function newright (val, right) {
  return newnode(val, null, right)
}

function newval (val) {
  return newnode(val, null, null)
}

/**
 * @param {TreeNode | null} p
 * @param {TreeNode | null} q
 * @returns {boolean}
 */
function isSameTree (p, q) {
  if ((p == null) && (q == null)) {
    return true
  }

  if ((p == null) || (q == null)) {
    return false
  }

  return p.val === q.val
    ? isSameTree(p.left, q.left) && isSameTree(p.right, q.right)
    : false
}

async function main () {
  const inputs = [
    [
      newnode(1, newval(2), newval(3)),
      newnode(1, newval(2), newval(3))
    ],
    [
      newleft(1, newval(2)),
      newright(1, newval(2))
    ],
    [
      newnode(1, newval(2), newval(1)),
      newnode(1, newval(1), newval(2))
    ]
  ]

  for (const [p, q] of inputs) {
    const result = isSameTree(p, q)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
