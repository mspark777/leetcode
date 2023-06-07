import '@total-typescript/ts-reset'

function countSetBits (n: number): number {
  let count = 0
  while (n > 0) {
    n &= (n - 1)
    count++
  }
  return count
}

function minFlips (a: number, b: number, c: number): number {
  const d = (a | b) ^ c
  const e = a & b & d
  return countSetBits(d) + countSetBits(e)
}

function main (): void {
  const inputs = [
    [2, 6, 5],
    [4, 2, 7],
    [1, 2, 3],
    [7, 3, 9]
  ]

  for (const [a, b, c] of inputs) {
    const result = minFlips(a, b, c)
    console.log(result)
  }
}
main()
