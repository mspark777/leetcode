function isPowerOfFour (n: number): boolean {
  const i = BigInt(n)
  return i > 0n && ((i & (i - 1n)) === 0n) && ((i & BigInt(0x55555555)) !== 0n)
}

interface Input {
  readonly n: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 16
    },
    {
      n: 5
    },
    {
      n: 1
    }
  ]

  for (const { n } of inputs) {
    const result = isPowerOfFour(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
