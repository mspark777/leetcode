import { UnionFind } from './utils'

function makeConnected (n: number, connections: number[][]): number {
  if (connections.length < (n - 1)) {
    return -1
  }

  const uf = new UnionFind(n)
  let result = n

  for (const [a, b] of connections) {
    if (uf.find(a) !== uf.find(b)) {
      result -= 1
      uf.union(a, b)
    }
  }

  return result - 1
}

interface Input {
  readonly n: number
  readonly connections: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: 4, connections: [[0, 1], [0, 2], [1, 2]] },
    { n: 6, connections: [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]] },
    { n: 6, connections: [[0, 1], [0, 2], [0, 3], [1, 2]] }
  ]

  for (const { n, connections } of inputs) {
    const result = makeConnected(n, connections)
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
