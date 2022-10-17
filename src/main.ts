function checkIfPangram (sentence: string): boolean {
  const ACODE = 'a'.charCodeAt(0)
  let bits = 0
  for (let i = 0; i < sentence.length; i += 1) {
    const code = sentence.charCodeAt(i)
    const offset = code - ACODE
    const bit = 1 << offset

    bits |= bit
  }

  return bits === 0x03ffffff
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'thequickbrownfoxjumpsoverthelazydog',
    'leetcode'
  ]

  for (const sentence of inputs) {
    const result = checkIfPangram(sentence)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
