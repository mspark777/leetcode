import '@total-typescript/ts-reset'

function minimumDeleteSum (s1: string, s2: string): number {
  const s1len = s1.length
  const s2len = s2.length

  if (s1len < s2len) {
    return minimumDeleteSum(s2, s1)
  }

  const curRow = new Array<number>(s2len + 1).fill(0)
  for (let i = 1; i <= s2len; i += 1) {
    const prev = curRow[i - 1] as number
    curRow[i] = prev + s2.charCodeAt(i - 1)
  }

  for (let i = 1; i <= s1len; i += 1) {
    let cur = curRow[0] as number
    curRow[0] += s1.charCodeAt(i - 1)

    for (let j = 1; j <= s2len; j += 1) {
      let col = 0
      if (s1.charCodeAt(i - 1) === s2.charCodeAt(j - 1)) {
        col = cur
      } else {
        col = Math.min(
          s1.charCodeAt(i - 1) + (curRow[j] as number),
          s2.charCodeAt(j - 1) + (curRow[j - 1] as number)
        )
      }

      cur = curRow[j] as number
      curRow[j] = col
    }
  }

  return curRow[s2len] as number
}

function main (): void {
  const inputs: string[][] = [
    ['sea', 'eat'],
    ['delete', 'leet']
  ]

  for (const [s1, s2] of inputs) {
    const result = minimumDeleteSum(s1 as string, s2 as string)
    console.log(result)
  }
}
main()
