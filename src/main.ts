import '@total-typescript/ts-reset'

function isNumber (s: string): boolean {
  const DOT = '.'
  const E = 'e'
  const DASH = '-'
  const PLUS = '+'
  let pointSeen = false
  let eSeen = false
  let numberSeen = false
  let numberAfterE = true
  const nums = new Set<string>(new Array(10).fill(0).map((_, i) => i.toString()))

  const chars = s.split('')
  for (const [i, ch] of chars.entries()) {
    if (nums.has(ch)) {
      numberSeen = true
      numberAfterE = true
    } else if (ch === DOT) {
      if (eSeen || pointSeen) {
        return false
      }

      pointSeen = true
    } else if (ch.toLowerCase() === E) {
      if (eSeen || !numberSeen) {
        return false
      }

      numberAfterE = false
      eSeen = true
    } else if ([DASH, PLUS].includes(ch)) {
      if (i === 0) {
        continue
      } else if (chars.at(i - 1)?.toLowerCase() !== E) {
        return false
      }
    } else {
      return false
    }
  }

  return numberSeen && numberAfterE
}

function main (): void {
  const inputs: string[] = [
    '0', 'e', '.', '1E9'
  ]

  for (const s of inputs) {
    const result = isNumber(s)
    console.log(result)
  }
}
main()
