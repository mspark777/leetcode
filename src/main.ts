function uniqueMorseRepresentations (words: string[]): number {
  const codes = [
    '.-', '-...', '-.-.', '-..', '.', '..-.', '--.', '....', '..', '.---', '-.-',
    '.-..', '--', '-.', '---', '.--.', '--.-', '.-.', '...', '-', '..-', '...-',
    '.--', '-..-', '-.--', '--..'
  ]
  const acode = 'a'.charCodeAt(0)
  const seen = new Set<string>()
  for (const word of words) {
    const wcodes: string[] = []
    for (let i = 0; i < word.length; i += 1) {
      const j = word.charCodeAt(i) - acode
      wcodes.push(codes[j])
    }

    seen.add(wcodes.join(''))
  }

  return seen.size
}

interface Input {
  readonly words: string[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      words: ['gin', 'zen', 'gig', 'msg']
    },
    {
      words: ['a']
    }
  ]

  for (const { words } of inputs) {
    const result = uniqueMorseRepresentations(words)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
