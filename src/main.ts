import '@total-typescript/ts-reset'

function convertToTitle (columnNumber: number): string {
  let n = BigInt(columnNumber)
  const result: string[] = []
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')
  const size = BigInt(chars.length)

  while (n > 0n) {
    n -= 1n

    const i = Number(n % size)
    const ch = chars[i] as string
    result.push(ch)

    n /= size
  }

  return result.reverse().join('')
}

function main (): void {
  const inputs: number[] = [
    1, 28, 701
  ]

  for (const columnNumber of inputs) {
    const result = convertToTitle(columnNumber)
    console.log(result)
  }
}
main()
