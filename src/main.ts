function transform (s: string): string {
  const result = new Array<number>(s.length)
  const indexMapping = new Map<string, number>()
  for (let i = 0; i < s.length; i += 1) {
    const ch = s.charAt(i)

    if (!indexMapping.has(ch)) {
      indexMapping.set(ch, i)
    }

    const idx = indexMapping.get(ch) as number
    result[i] = idx
  }

  return result.join(' ')
}

function isIsomorphic (s: string, t: string): boolean {
  return transform(s) === transform(t)
}

interface Input {
  readonly s: string
  readonly t: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      s: 'egg',
      t: 'add'
    },
    {
      s: 'foo',
      t: 'bar'
    },
    {
      s: 'paper',
      t: 'title'
    }
  ]

  for (const { s, t } of inputs) {
    const result = isIsomorphic(s, t)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
