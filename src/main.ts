import '@total-typescript/ts-reset'

function multiply (num1: string, num2: string): string {
  const result = BigInt(num1) * BigInt(num2)
  return result.toString()
}

function main (): void {
  const inputs: string[][] = [
    ['2', '3'],
    ['123', '456']
  ]

  for (const [num1, num2] of inputs) {
    const result = multiply(num1 as string, num2 as string)
    console.log(result)
  }
}
main()
