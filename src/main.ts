function canConstruct (ransomNote: string, magazine: string): boolean {
  const counts = new Map<number, number>()
  for (let i = 0; i < magazine.length; i += 1) {
    const code = magazine.charCodeAt(i)
    const cnt = counts.get(code) ?? 0
    counts.set(code, cnt + 1)
  }

  for (let i = 0; i < ransomNote.length; i += 1) {
    const code = ransomNote.charCodeAt(i)
    const cnt = counts.get(code) ?? 0
    if (cnt < 1) {
      return false
    }
    counts.set(code, cnt - 1)
  }

  return true
}

interface Input {
  readonly ransomNote: string
  readonly magazine: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      ransomNote: 'a',
      magazine: 'b'
    },
    {
      ransomNote: 'aa',
      magazine: 'ab'
    },
    {
      ransomNote: 'aa',
      magazine: 'aab'
    }
  ]

  for (const { ransomNote, magazine } of inputs) {
    const result = canConstruct(ransomNote, magazine)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
