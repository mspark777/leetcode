import '@total-typescript/ts-reset'

function div (a: number, b: number): number {
  return Number(BigInt(a) / BigInt(b))
}

function mod (a: number, b: number): number {
  return Number(BigInt(a) % BigInt(b))
}

function searchMatrix (matrix: number[][], target: number): boolean {
  const rowCount = matrix.length
  const colCount = matrix.at(0)?.length as number
  let left = 0
  let right = (rowCount * colCount) - 1

  while (left <= right) {
    const mid = div(left + right, 2)
    const row = div(mid, colCount)
    const col = mod(mid, colCount)
    const guess = matrix.at(row)?.at(col) as number
    if (guess < target) {
      left = mid + 1
    } else if (guess > target) {
      right = mid - 1
    } else {
      return true
    }
  }

  return false
}

function main (): void {
  const inputs: Array<[number[][], number]> = [
    [[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], 3],
    [[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], 13]
  ]

  for (const [matrix, target] of inputs) {
    const result = searchMatrix(matrix, target)
    console.log(result)
  }
}
main()
