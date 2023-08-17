import '@total-typescript/ts-reset'

function updateMatrix (mat: number[][]): number[][] {
  const rowCount = mat.length
  const colCount = mat[0]?.length as number
  const result: number[][] = new Array(rowCount)
  for (let r = 0; r < rowCount; r += 1) {
    result[r] = new Array(colCount).fill(0)
  }

  const queue: number[][] = []
  const maxValue = rowCount * colCount

  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if (mat.at(r)?.at(c) === 0) {
        queue.push([r, c])
      } else {
        const row = result[r] as number[]
        row[c] = maxValue
      }
    }
  }

  const directions = [[1, 0], [0, 1], [-1, 0], [0, -1]]
  for (let head = queue.shift(); head != null; head = queue.shift()) {
    const [row, col] = head as [number, number]
    const cell0 = (result.at(row)?.at(col) as number) + 1

    for (const dir of directions) {
      const [dr, dc] = dir as [number, number]
      const r = row + dr
      const c = col + dc
      if (r < 0) {
        continue
      } else if (r >= rowCount) {
        continue
      } else if (c < 0) {
        continue
      } else if (c >= colCount) {
        continue
      }

      const cell1 = result.at(r)?.at(c) as number
      if (cell1 <= cell0) {
        continue
      }

      queue.push([r, c])
      const rrow = result[r] as number[]
      rrow[c] = cell0
    }
  }

  return result
}

function main (): void {
  const inputs: number[][][] = [
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 0, 0], [0, 1, 0], [1, 1, 1]]
  ]

  for (const mat of inputs) {
    const result = updateMatrix(mat)
    console.log(result)
  }
}
main()
