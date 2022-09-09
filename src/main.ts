function isUgly (n: number): boolean {
  let i = BigInt(n)
  while (i > 1n) {
    if ((i % 2n) === 0n) {
      i /= 2n
    } else if ((i % 3n) === 0n) {
      i /= 3n
    } else if ((i % 5n) === 0n) {
      i /= 5n
    } else {
      return false
    }
  }

  return i === 1n
}

interface Input {
  readonly n: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 6
    },
    {
      n: 1
    },
    {
      n: 14
    },
    {
      n: -2147483648
    }
  ]

  for (const { n } of inputs) {
    const result = isUgly(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
