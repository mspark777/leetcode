function minimumLengthEncoding (words) {
  const filter = new Set(words)
  for (const word of words) {
    for (let i = 1; i < word.length; i += 1) {
      filter.delete(word.substring(i))
    }
  }

  let result = 0
  for (const word of filter) {
    result += word.length + 1
  }

  return result
}

async function main () {
  const inputs = [
    ['time', 'me', 'bell'],
    ['t']
  ]

  for (const input of inputs) {
    const result = minimumLengthEncoding(input)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
