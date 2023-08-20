import '@total-typescript/ts-reset'

function get (m: number[][], r: number, c: number): number {
  return m.at(r)?.at(c) as number
}

function set (m: number[][], r: number, c: number, v: number): void {
  const row = m.at(r) as number[]
  row[c] = v
}

/**
 Do not return anything, modify matrix in-place instead.
 */
function setZeroes (matrix: number[][]): void {
  const rowCount = matrix.length
  const colCount = matrix.at(0)?.length as number
  let fr = false
  let fc = false
  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if (get(matrix, r, c) === 0) {
        fr ||= r === 0
        fc ||= c === 0
        set(matrix, 0, c, 0)
        set(matrix, r, 0, 0)
      }
    }
  }

  for (let r = 1; r < rowCount; r += 1) {
    for (let c = 1; c < colCount; c += 1) {
      if (get(matrix, r, 0) === 0) {
        set(matrix, r, c, 0)
      } else if (get(matrix, 0, c) === 0) {
        set(matrix, r, c, 0)
      }
    }
  }

  if (fr) {
    for (let c = 0; c < colCount; c += 1) {
      set(matrix, 0, c, 0)
    }
  }

  if (fc) {
    for (let r = 0; r < rowCount; r += 1) {
      set(matrix, r, 0, 0)
    }
  }
}

function main (): void {
  const inputs: number[][][] = [
    [[1, 1, 1], [1, 0, 1], [1, 1, 1]],
    [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]]
  ]

  for (const matrix of inputs) {
    setZeroes(matrix)
    console.log(matrix)
  }
}
main()
