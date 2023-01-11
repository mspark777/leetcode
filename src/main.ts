function dfs(node: number, prevNode: number, adjMat: number[][], hasApple: boolean[]): number {
  let totalTime = 0
  let childTime = 0

  for (const child of adjMat[node]) {
    if (child === prevNode) {
      continue
    }

    childTime = dfs(child, node, adjMat, hasApple)
    if ((childTime > 0) || hasApple[child]) {
      totalTime += childTime + 2
    }
  }

  return totalTime
}

function minTime(n: number, edges: number[][], hasApple: boolean[]): number {
  const adjMat = Array.from(new Array<number[]>(n), () => new Array<number>())
  for (const [l, r] of edges) {
    adjMat[l].push(r)
    adjMat[r].push(l)
  }

  return dfs(0, -1, adjMat, hasApple)
}

interface Input {
  readonly n: number
  readonly edges: number[][]
  readonly hasApple: boolean[]
}
async function main(): Promise<void> {
  const inputs: Input[] = [
    {n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]], hasApple: [false, false, true, false, true, true, false]},
    {n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]], hasApple: [false, false, true, false, false, true, false]},
    {n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]], hasApple: [false, false, false, false, false, false, false]}
  ]

  for (const {n, edges, hasApple} of inputs) {
    const result = minTime(n, edges, hasApple)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
