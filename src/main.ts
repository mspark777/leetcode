import '@total-typescript/ts-reset'

function myPow1 (x: number, n: bigint): number {
  if (n === 0n) {
    return 1
  }

  if (n < 0n) {
    n *= -1n
    x = 1 / x
  }

  let result = 1
  while (n !== 0n) {
    if ((n % 2n) === 1n) {
      result *= x
      n -= 1n
    }

    x *= x
    n /= 2n
  }

  return result
}

function myPow (x: number, n: number): number {
  return myPow1(x, BigInt(n))
}

interface Input {
  readonly x: number
  readonly n: number
}

function main (): void {
  const inputs: Input[] = [
    { x: 2.00000, n: 10 },
    { x: 2.10000, n: 3 },
    { x: 2.00000, n: -2 }
  ]

  for (const { x, n } of inputs) {
    const result = myPow(x, n)
    console.log(result)
  }
}
main()
