function isPowerOfThree (n: number): boolean {
  let i = BigInt(n)
  if (i <= 0n) {
    return false
  }

  while ((i % 3n) === 0n) {
    i /= 3n
  }

  return i === 1n
}

interface Input {
  readonly n: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 27
    },
    {
      n: 0
    },
    {
      n: 9
    },
    {
      n: 45
    }
  ]

  for (const { n } of inputs) {
    const result = isPowerOfThree(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
