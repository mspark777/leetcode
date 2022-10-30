function shortestPath (grid: number[][], k: number): number {
  const M = grid.length
  const N = grid[0].length
  const visits = new Array<number[]>(M)
  for (let i = 0; i < M; i += 1) {
    visits[i] = new Array<number>(N).fill(-1)
  }

  const LM = M - 1
  const LN = N - 1
  const queue: number[][] = [[0, 0, 0, k]]
  for (let head = queue.shift(); head != null; head = queue.shift()) {
    const [x, y, step] = head

    if ((x < 0) || (y < 0) || (x >= M) || (y >= N)) {
      continue
    }

    if ((x === LM) && (y === LN)) {
      return step
    }

    let chance = head[3]
    if (grid[x][y] === 1) {
      if (chance > 0) {
        chance -= 1
      } else {
        continue
      }
    }

    if ((visits[x][y] !== -1) && (visits[x][y] >= chance)) {
      continue
    }

    visits[x][y] = chance

    const next = step + 1
    queue.push(
      [x + 1, y, next, chance],
      [x, y + 1, next, chance],
      [x - 1, y, next, chance],
      [x, y - 1, next, chance]
    )
  }

  return -1
}

interface Input {
  readonly grid: number[][]
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      grid: [[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]],
      k: 1
    },
    {
      grid: [[0, 1, 1], [1, 1, 1], [1, 0, 0]],
      k: 1
    }
  ]

  for (const { grid, k } of inputs) {
    const result = shortestPath(grid, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
