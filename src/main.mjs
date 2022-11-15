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
 * @param {TreeNode | null} root
 * @returns {number}
*/
function getHeight (root) {
  return root != null
    ? 1 + getHeight(root.left)
    : -1
}

/**
 * @param {TreeNode | null} root
 * @returns {number}
*/
function countNodes (root) {
  let nodes = 0
  let h = getHeight(root)
  while (root != null) {
    const next = h - 1
    if (getHeight(root.right) === next) {
      nodes += 1 << h
      root = root.right
    } else {
      nodes += 1 << next
      root = root.left
    }
    h = next
  }

  return nodes
}

async function main () {
  const inputs = [
    newnode(1, newnode(2, newval(4), newval(5)), newleft(3, newval(6))),
    null,
    newval(1),
    newnode(1, newleft(2, newval(4)), newval(3))
  ]

  for (const root of inputs) {
    const result = countNodes(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
