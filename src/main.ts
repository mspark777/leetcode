function removeDuplicates (s: string): string {
  const result: string[] = []

  for (const c of s) {
    if (result.at(-1) === c) {
      result.pop()
    } else {
      result.push(c)
    }
  }

  return result.join('')
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'abbaca',
    'azxxzy'
  ]

  for (const s of inputs) {
    const result = removeDuplicates(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
