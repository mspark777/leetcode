/**
 * @param {number[][]} graph
 * @return {boolean}
 */
export const isBipartite1 = function (graph) {
  const NONE = 0
  const RED = 1
  const colors = graph.map(_ => NONE)
  const stack = []
  for (let i = 0; i < graph.length; i += 1) {
    if (colors[i] !== NONE) {
      continue
    }

    colors[i] = RED
    stack.push(i)
    while (stack.length > 0) {
      const current = stack.pop()
      const currentColor = colors[current]

      for (const vertex of graph[current]) {
        const vertexColor = colors[vertex]
        if (vertexColor === NONE) {
          colors[vertex] = -currentColor
          stack.push(vertex)
        } else if (currentColor === vertexColor) {
          return false
        }
      }
    }
  }

  return true
}
