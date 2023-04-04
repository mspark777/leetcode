function partitionString (s: string): number {
  const seens = new Array(26).fill(-1)
  let count = 1
  let substart = 0

  const ACODE = 'a'.charCodeAt(0)
  for (let i = 0; i < s.length; i += 1) {
    const code = s.charCodeAt(i) - ACODE
    if (seens[code] >= substart) {
      count += 1
      substart = i
    }
    seens[code] = i
  }

  return count
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'abacaba',
    'ssssss'
  ]

  for (const s of inputs) {
    const result = partitionString(s)
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
