function gcd (x: number, y: number): number {
  return y !== 0 ? gcd(y, x % y) : x
}

function gcdOfStrings (str1: string, str2: string): string {
  if (`${str1}${str2}` !== `${str2}${str1}`) {
    return ''
  }

  const gcdLen = gcd(str1.length, str2.length)
  return str1.substring(0, gcdLen)
}

async function main (): Promise<void> {
  const inputs: string[][] = [
    ['ABCABC', 'ABC'],
    ['ABABAB', 'ABAB'],
    ['LEET', 'CODE']
  ]

  for (const [str1, str2] of inputs) {
    const result = gcdOfStrings(str1, str2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
