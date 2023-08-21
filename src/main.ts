import '@total-typescript/ts-reset'

function div (a: number, b: number): number {
  return Number(BigInt(a) / BigInt(b))
}

function mod (a: number, b: number): number {
  return Number(BigInt(a) % BigInt(b))
}

function repeatedSubstringPattern (s: string): boolean {
  for (let i = 1; i <= div(s.length, 2); i += 1) {
    if (mod(s.length, i) !== 0) {
      continue
    }

    const pattern = s.substring(0, i).repeat(div(s.length, i))
    if (pattern === s) {
      return true
    }
  }

  return false
}

function main (): void {
  const inputs: string[] = [
    'abab',
    'aba',
    'abcabcabcabc'
  ]

  for (const s of inputs) {
    const result = repeatedSubstringPattern(s)
    console.log(result)
  }
}
main()
