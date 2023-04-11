function removeStars (s: string): string {
  const stack: string[] = []
  for (const ch of s) {
    if (ch === '*') {
      stack.pop()
    } else {
      stack.push(ch)
    }
  }

  return stack.join('')
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'leet**cod*e',
    'erase*****'
  ]

  for (const s of inputs) {
    const result = removeStars(s)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
