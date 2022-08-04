function mirrorReflection (p: number, q: number): number {
  while (((p % 2) + (q % 2)) === 0) {
    p = Math.trunc(p / 2)
    q = Math.trunc(q / 2)
  }

  return (q % 2) - (p % 2) + 1
}

interface Input {
  readonly p: number
  readonly q: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      p: 2,
      q: 1
    },
    {
      p: 3,
      q: 1
    }
  ]

  for (const { p, q } of inputs) {
    const result = mirrorReflection(p, q)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
