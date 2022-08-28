function diagonalSort (mat: number[][]): number[][] {
  const rowCount = mat.length
  const colCount = mat[0].length

  const queues = new Map<number, number[]>()
  for (let i = 0; i < rowCount; i += 1) {
    for (let j = 0; j < colCount; j += 1) {
      const key = i - j
      const queue = queues.get(key) ?? []
      queue.push(mat[i][j])
      queues.set(key, queue)
    }
  }

  for (const queue of queues.values()) {
    queue.sort((a, b) => b - a)
  }

  const result = new Array<number[]>(rowCount)
  for (let i = 0; i < rowCount; i += 1) {
    const row = new Array<number>(colCount)
    for (let j = 0; j < colCount; j += 1) {
      const key = i - j
      const queue = queues.get(key) as number[]
      row[j] = queue.pop() as number
    }

    result[i] = row
  }

  return result
}

interface Input {
  readonly mat: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      mat: [[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]]
    },
    {
      mat: [[11, 25, 66, 1, 69, 7], [23, 55, 17, 45, 15, 52], [75, 31, 36, 44, 58, 8], [22, 27, 33, 25, 68, 4], [84, 28, 14, 11, 5, 50]]
    }
  ]

  for (const { mat } of inputs) {
    const result = diagonalSort(mat)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
