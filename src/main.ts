import '@total-typescript/ts-reset'

function isMatch (s: string, p: string): boolean {
  let si = 0
  let pi = 0
  let mi = 0
  let starIdx = -1

  while (si < s.length) {
    if ((pi < p.length) && ((p.charAt(pi) === '?') || (s.charAt(si) === p.charAt(pi)))) {
      si += 1
      pi += 1
    } else if ((pi < p.length) && (p.charAt(pi) === '*')) {
      starIdx = pi
      mi = si
      pi += 1
    } else if (starIdx >= 0) {
      pi = starIdx + 1
      mi += 1
      si = mi
    } else {
      return false
    }
  }

  while (pi < p.length) {
    if (p.charAt(pi) === '*') {
      pi += 1
    } else {
      break
    }
  }

  return pi === p.length
}

function main (): void {
  const inputs: string[][] = [
    ['aa', 'a'],
    ['aa', '*'],
    ['cb', '?a']
  ]

  for (const [s, p] of inputs) {
    const result = isMatch(s as string, p as string)
    console.log(result)
  }
}
main()
