import '@total-typescript/ts-reset'

function ncr (n: bigint, r: bigint): bigint {
  let result = 1n
  for (let i = 0n; i < r; i += 1n) {
    result *= n - i
    result /= i + 1n
  }

  return result
}

function numTrees (n: number): number {
  const num = BigInt(n)
  return Number(ncr(2n * num, num) / (num + 1n))
}

function main (): void {
  const inputs: number[] = [
    3, 1, 4
  ]

  for (const n of inputs) {
    const result = numTrees(n)
    console.log(result)
  }
}
main()
