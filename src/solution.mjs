export function generate (numRows) {
  const result = []

  for (let i = 0; i < numRows; i += 1) {
    const row = new Array(i + 1)
    row[0] = 1
    row[i] = 1
    const prev = i - 1
    for (let j = 1; j < i; j += 1) {
      row[j] = result[prev][j - 1] + result[prev][j]
    }
    result.push(row)
  }

  return result
}
