/**
 * @param {number} node
 * @param {number} prevNode
 * @param {number[][]} adjMat
 * @param {string} labels
 * @param {Map<number, number[]>} seens
 * @param {number[]} result
 * @returns {number[]}
 */
function dfs (node, prevNode, adjMat, labels, seens, result) {
  if (seens.has(node)) {
    return seens.get(node)
  }

  const LEN = 26
  const ACODE = 'a'.charCodeAt(0)
  const counts = new Array(LEN).fill(0)
  counts[labels.charCodeAt(node) - ACODE] = 1

  for (const child of adjMat[node]) {
    if (child === prevNode) {
      continue
    }

    const childCounts = dfs(child, node, adjMat, labels, seens, result)
    for (let i = 0; i < LEN; i += 1) {
      counts[i] += childCounts[i]
    }
  }

  result[node] = counts[labels.charCodeAt(node) - ACODE]
  seens.set(node, counts)
  return counts
}

/**
 * @param {number} n
 * @param {number[][]} edges
 * @param {string} labels
 * @returns {number[]}
 */
function countSubTrees (n, edges, labels) {
  const adjMat = Array.from(new Array(n), () => [])
  for (const [l, r] of edges) {
    adjMat[l].push(r)
    adjMat[r].push(l)
  }

  const result = new Array(n).fill(0)
  dfs(0, -1, adjMat, labels, new Map(), result)

  return result
}

async function main () {
  const inputs = [
    { n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]], labels: 'abaedcd' },
    { n: 4, edges: [[0, 1], [1, 2], [0, 3]], labels: 'bbbb' },
    { n: 5, edges: [[0, 1], [0, 2], [1, 3], [0, 4]], labels: 'aabab' }
  ]

  for (const { n, edges, labels } of inputs) {
    const result = countSubTrees(n, edges, labels)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
