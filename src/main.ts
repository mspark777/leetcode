import '@total-typescript/ts-reset'

class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor (val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function newnode (val: number, left?: TreeNode | null, right?: TreeNode | null): TreeNode {
  return new TreeNode(val, left, right)
}

function newleft (val: number, left?: TreeNode | null): TreeNode {
  return newnode(val, left)
}

function newright (val: number, right?: TreeNode | null): TreeNode {
  return newnode(val, undefined, right)
}

function newval (val: number): TreeNode {
  return newnode(val)
}

function buildGraph (graph: Map<number, number[]>, node: TreeNode | null, parent: TreeNode | null): void {
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

function dfs (graph: Map<number, number[]>, visited: Set<number>, current: number, distance: number, target: number, result: number[]): void {
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

function distanceK (root: TreeNode | null, target: TreeNode | null, k: number): number[] {
  if (root == null) {
    return []
  } else if (target == null) {
    return []
  }

  const graph = new Map<number, number[]>()
  buildGraph(graph, root, null)

  const result: number[] = []
  const visited = new Set<number>([target.val])
  dfs(graph, visited, target.val, 0, k, result)

  return result
}

interface Input {
  readonly root: TreeNode
  readonly target: TreeNode
  readonly k: number
}

function main (): void {
  const inputs: Input[] = [
    { root: newnode(3, newnode(5, newval(6), newnode(2, newval(7), newval(4))), newnode(1, newval(0), newval(8))), target: newval(5), k: 2 },
    { root: newval(1), target: newval(1), k: 3 }
  ]

  for (const { root, target, k } of inputs) {
    const result = distanceK(root, target, k)
    console.log(result)
  }
}
main()
