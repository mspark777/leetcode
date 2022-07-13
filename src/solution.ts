export function getRow (rowIndex: number): number[] {
  const result = new Array<number>(rowIndex + 1).fill(1)

  for (let i = 0; i <= rowIndex; i += 1) {
    for (let j = i - 1; j > 0; j -= 1) {
      result[j] += result[j - 1]
    }
  }

  return result
}
