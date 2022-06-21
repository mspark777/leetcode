export function hammingWeight (n: number): number {
  let count = 0
  while (n !== 0) {
    n = n & (n - 1)
    count += 1
  }

  return count
}

export function hammingWeight1 (n: number): number {
  let count = 0
  while (n !== 0) {
    n &= n - 1
    count += 1
  }

  return count
}
