class TreeNode {
  val
  left
  right
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
  return newnode(val, left)
}

function newright (val, right) {
  return newnode(val, undefined, right)
}

function newval (val) {
  return newnode(val)
}

function treetoarr (node) {
  const nums = []
  const travel = n => {
    if (n != null) {
      nums.push(n.val)
      travel(n.left)
      travel(n.right)
    }
  }
  travel(node)

  return nums
}

/**
 * @param {TreeNode} root
 * @param {number} val
 * @param {number} depth
 * @returns {TreeNode}
 */
function addOneRow (root, val, depth) {
  if (root == null) {
    return null
  }

  if (depth === 1) {
    return new TreeNode(val, root)
  }

  const stack = [{
    node: root,
    pos: 2
  }]

  const targets = []
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { pos, node } = top
    if (pos > depth) {
      continue
    }

    if (pos === depth) {
      targets.push(node)
      continue
    }

    const { left, right } = node
    if (left != null) {
      stack.push({
        node: left,
        pos: pos + 1
      })
    }

    if (right != null) {
      stack.push({
        node: right,
        pos: pos + 1
      })
    }
  }

  for (const target of targets) {
    target.left = new TreeNode(val, target.left)
    target.right = new TreeNode(val, undefined, target.right)
  }

  return root
}

async function main () {
  const inputs = [
    {
      root: newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
      val: 1,
      depth: 2
    },
    {
      root: newleft(4, newnode(2, newval(3), newval(1))),
      val: 1,
      depth: 3
    },
    {
      root: newnode(4, newnode(2, newval(3), newval(1)), newleft(6, newval(5))),
      val: 1,
      depth: 1
    }
  ]

  for (const { root, val, depth } of inputs) {
    const result = addOneRow(root, val, depth)
    console.log(treetoarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
