function isPowerOfTwo (n: number): boolean {
  const b = BigInt(n)
  return b <= 0 ? false : (b & (b - 1n)) === 0n
}

interface Input {
  readonly n: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 1
    },
    {
      n: 16
    },
    {
      n: 3
    }
  ]

  for (const { n } of inputs) {
    const result = isPowerOfTwo(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
