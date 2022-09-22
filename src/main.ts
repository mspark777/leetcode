function reverseWords (s: string): string {
  return s.split(' ').map(ss => ss.split('').reverse().join('')).join(' ')
}

interface Input {
  readonly s: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      s: "Let's take LeetCode contest"
    },
    {
      s: 'God Ding'
    }
  ]

  for (const { s } of inputs) {
    const result = reverseWords(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
