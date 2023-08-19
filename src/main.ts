import '@total-typescript/ts-reset'

function div (a: number, b: number): number {
  return Number(BigInt(a) / BigInt(b))
}

function mod (a: number, b: number): number {
  return Number(BigInt(a) % BigInt(b))
}

function getWords (i: number, words: string[], maxWidth: number): string[] {
  const line: string[] = []
  let lineLength = 0
  while (i < words.length) {
    const word = words.at(i) as string
    const len = lineLength + word.length
    if (len > maxWidth) {
      break
    }

    line.push(word)
    lineLength += word.length + 1
    i += 1
  }

  return line
}

function createLine (line: string[], i: number, words: string[], maxWidth: number): string {
  let baseLength = -1
  for (const word of line) {
    baseLength += word.length + 1
  }

  const extraSpaces = maxWidth - baseLength
  if ((line.length === 1) || (words.length === i)) {
    return line.join(' ') + ' '.repeat(extraSpaces)
  }

  const wordCount = line.length - 1
  const spacesPerWord = div(extraSpaces, wordCount)
  const needsExtraSpace = mod(extraSpaces, wordCount)
  for (let j = 0; j < needsExtraSpace; j += 1) {
    const w = line[j] as string
    line[j] = w + ' '
  }

  for (let j = 0; j < wordCount; j += 1) {
    const w = line[j] as string
    line[j] = w + ' '.repeat(spacesPerWord)
  }

  return line.join(' ')
}

function fullJustify (words: string[], maxWidth: number): string[] {
  const result: string[] = []
  let i = 0
  while (i < words.length) {
    const line = getWords(i, words, maxWidth)
    i += line.length
    result.push(createLine(line, i, words, maxWidth))
  }

  return result
}

function main (): void {
  const inputs: Array<[string[], number]> = [
    [['This', 'is', 'an', 'example', 'of', 'text', 'justification.'], 16],
    [['What', 'must', 'be', 'acknowledgment', 'shall', 'be'], 16],
    [['Science', 'is', 'what', 'we', 'understand', 'well', 'enough', 'to', 'explain', 'to', 'a', 'computer.', 'Art', 'is', 'everything', 'else', 'we', 'do'], 20]
  ]

  for (const [words, maxWidth] of inputs) {
    const result = fullJustify(words, maxWidth)
    console.log(result)
  }
}
main()
