class TreeNode {
  /* @type {number} */
  val
  /* @type {TreeNode|null} */
  left
  /* @type {TreeNode|null} */
  right
  /**
    * @param {number} [val]
    * @param {TreeNode|null} [left]
    * @param {TreeNode|null} [right]
    */
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

/**
  * @param {Map<number, number[]>} graph
  * @param {TreeNode|null} node
  * @param {TreeNode|null} parent
  * @returns {undefined}
  */
function buildGraph (graph, node, parent) {
  if (node == null) {
    return
  }

  if (parent != null) {
    const currAbsents = graph.get(node.val) ?? []
    currAbsents.push(parent.val)
    graph.set(node.val, currAbsents)

    const parentAbsents = graph.get(parent.val) ?? []
    parentAbsents.push(node.val)
    graph.set(parent.val, parentAbsents)
  }

  if (node.left != null) {
    buildGraph(graph, node.left, node)
  }

  if (node.right != null) {
    buildGraph(graph, node.right, node)
  }
}

/**
  * @param {Map<number, number[]>} graph
  * @param {Set<number>} visited
  * @param {number} current
  * @param {number} distance
  * @param {number} target
  * @param {number[]} result
  * @returns {undefined}
  */
function dfs (graph, visited, current, distance, target, result) {
  if (distance === target) {
    result.push(current)
    return
  }

  const next = distance + 1
  for (const neighbor of graph.get(current) ?? []) {
    if (visited.has(neighbor)) {
      continue
    }

    visited.add(neighbor)
    dfs(graph, visited, neighbor, next, target, result)
  }
}

/**
  * @param {TreeNode|null} root
  * @param {TreeNode|null} target
  * @param {number} k
  * @returns {number[]}
  */
function distanceK (root, target, k) {
  if (root == null) {
    return []
  } else if (target == null) {
    return []
  }

  /** @type {Map<number, number[]>} */
  const graph = new Map()
  buildGraph(graph, root, null)

  /** @type {number[]} */
  const result = []
  /** @type {Set<number>}  */
  const visited = new Set([target.val])
  dfs(graph, visited, target.val, 0, k, result)

  return result
}

function main () {
  const inputs = [
    { root: newnode(3, newnode(5, newval(6), newnode(2, newval(7), newval(4))), newnode(1, newval(0), newval(8))), target: newval(5), k: 2 },
    { root: newval(1), target: newval(1), k: 3 }
  ]

  for (const { root, target, k } of inputs) {
    const result = distanceK(root, target, k)
    console.log(result)
  }
}
main()
