function dfs (node: number, prevNode: number, adjMat: number[][], labels: string, seens: Map<number, number[]>, result: number[]): number[] {
  if (seens.has(node)) {
    return seens.get(node) as number[]
  }

  const LEN = 26
  const ACODE = 'a'.charCodeAt(0)
  const counts = new Array<number>(LEN).fill(0)
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

function countSubTrees (n: number, edges: number[][], labels: string): number[] {
  const adjMat = Array.from(new Array<number[]>(n), () => new Array<number>())
  for (const [l, r] of edges) {
    adjMat[l].push(r)
    adjMat[r].push(l)
  }

  const result = new Array(n).fill(0)
  dfs(0, -1, adjMat, labels, new Map(), result)

  return result
}

interface Input {
  readonly n: number
  readonly edges: number[][]
  readonly labels: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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
