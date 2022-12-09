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
 * @param {TreeNode | null} node
 * @param {number} curmax
 * @param {number} curmin
 * @returns {number}
*/
function travel (node, curmax, curmin) {
  if (node == null) {
    return curmax - curmin
  }

  const { val, left, right } = node
  curmax = Math.max(curmax, val)
  curmin = Math.min(curmin, val)

  const l = travel(left, curmax, curmin)
  const r = travel(right, curmax, curmin)
  return Math.max(l, r)
}

/**
 * @param {TreeNode | null} root
 * @returns {number}
*/
function maxAncestorDiff (root) {
  return root != null ? travel(root, root.val, root.val) : 0
}

async function main () {
  const inputs = [
    newnode(8,
      newnode(3,
        newval(1),
        newnode(6,
          newval(4),
          newval(7)
        )
      ),
      newright(10,
        newleft(14,
          newval(13)
        )
      )
    ),
    newright(1,
      newright(2,
        newleft(0,
          newval(3)
        )
      )
    )
  ]

  for (const root of inputs) {
    const result = maxAncestorDiff(root)
    console.log(result)
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})
