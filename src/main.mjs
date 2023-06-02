/**
  * @param {number} i
  * @param {Map<number, number[]>} graph
  * @returns number
  */
function dfs (i, graph) {
  const stack = [i]
  const visited = new Set([i])

  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const nodes = graph.get(top) ?? []
    for (const node of nodes) {
      if (!visited.has(node)) {
        stack.push(node)
        visited.add(node)
      }
    }
  }

  return visited.size
}

/**
  * @param {number[][]} bombs
  * @returns {number}
  */
function maximumDetonation (bombs) {
  /** @type {Map<number, number[]> */
  const graph = new Map()

  for (const [i, ibomb] of bombs.entries()) {
    for (const [j, jbomb] of bombs.entries()) {
      if (i === j) {
        continue
      }

      const [ix, iy, ir] = ibomb
      const [jx, jy] = jbomb
      const dx = ix - jx
      const dy = iy - jy
      const range = ir * ir
      const distance = (dx * dx) + (dy * dy)
      if (range < distance) {
        continue
      }

      const nodes = graph.get(i) ?? []
      nodes.push(j)
      graph.set(i, nodes)
    }
  }

  let result = 0
  for (let i = 0; i < bombs.length; i += 1) {
    result = Math.max(result, dfs(i, graph))
  }

  return result
}

function main () {
  const inputs = [
    [[2, 1, 3], [6, 1, 4]],
    [[1, 1, 5], [10, 10, 5]],
    [[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]]
  ]

  for (const bombs of inputs) {
    const result = maximumDetonation(bombs)
    console.log(result)
  }
}
main()
