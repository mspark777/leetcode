function intToRoman (num: number): string {
  const M: string[] = ['', 'M', 'MM', 'MMM']
  const C: string[] = ['', 'C', 'CC', 'CCC', 'CD', 'D', 'DC', 'DCC', 'DCCC', 'CM']
  const X: string[] = ['', 'X', 'XX', 'XXX', 'XL', 'L', 'LX', 'LXX', 'LXXX', 'XC']
  const I: string[] = ['', 'I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX']

  const n = BigInt(num)
  const n1000 = 1000n
  const n100 = 100n
  const n10 = 10n
  const mi = Number(n / n1000)
  const ci = Number((n % n1000) / n100)
  const xi = Number((n % n100) / n10)
  const ii = Number(n % n10)
  return `${M[mi]}${C[ci]}${X[xi]}${I[ii]}`
}

async function main (): Promise<void> {
  const inputs: number[] = [3, 58, 1994]

  for (const num of inputs) {
    const result = intToRoman(num)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
