function addDigits (num: number): number {
  return num === 0 ? 0 : 1 + (num - 1) % 9
}

interface Input {
  readonly num: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      num: 38
    },
    {
      num: 0
    }
  ]

  for (const { num } of inputs) {
    const result = addDigits(num)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
