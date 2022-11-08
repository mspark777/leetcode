function makeGood (s: string): string {
  const chars = [...s]

  let j = 0
  for (let i = 0; i < chars.length; i += 1) {
    if (j > 0) {
      const cur = chars[i]
      const next = chars[j - 1]
      const diff = Math.abs(cur.charCodeAt(0) - next.charCodeAt(0))
      if (diff === 32) {
        j -= 1
        continue
      }
    }

    chars[j] = chars[i]
    j += 1
  }

  return chars.slice(0, j).join('')
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'leEeetcode',
    'abBAcC',
    's'
  ]

  for (const s of inputs) {
    const result = makeGood(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
