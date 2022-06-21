/**
 * @param {number[][]} graph
 * @return {boolean}
 */
export const isBipartite = function (graph) {
  const NONE = -1
  const RED = 1
  const BLUE = 2
  const visit = (pos, colors, myColor, willColor) => {
    const verties = graph[pos]
    for (const vertex of verties) {
      const youColor = colors[vertex]
      if (myColor === youColor) {
        return false
      }

      if (youColor === NONE) {
        colors[vertex] = willColor
        if (!visit(vertex, colors, willColor, myColor)) {
          return false
        }
      }
    }

    return true
  }

  const colors = graph.map(_ => NONE)
  for (let i = 0; i < graph.length; i += 1) {
    for (const j in colors) {
      colors[j] = NONE
    }

    colors[i] = RED
    if (!visit(i, colors, RED, BLUE)) {
      return false
    }
  }

  return true
}
