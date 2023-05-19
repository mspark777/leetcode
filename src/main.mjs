/**
  * @param {number[][]} graph
  * @returns {boolean}
  */
function isBipartite (graph) {
  const NONE = 0
  const RED = 1
  // const BLUE = -1
  const colors = new Array(graph.length).fill(NONE)
  const stack = []
  for (let i = 0; i < graph.length; i += 1) {
    if (colors[i] !== NONE) {
      continue
    }

    colors[i] = RED
    stack.push(i)
    for (let vertex = stack.pop(); vertex != null; vertex = stack.pop()) {
      const color = colors[vertex]
      for (const adjacent of graph[vertex]) {
        const acolor = colors[adjacent]
        if (acolor === NONE) {
          colors[adjacent] = -color
          stack.push(adjacent)
        } else if (color === acolor) {
          return false
        }
      }
    }
  }

  return true
}

async function main () {
  const inputs = [
    [[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]],
    [[1, 3], [0, 2], [1, 3], [0, 2]]
  ]

  for (const graph of inputs) {
    const result = isBipartite(graph)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
