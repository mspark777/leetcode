function romanToInt (s: string): number {
  let result = 0
  let num = 0
  for (let i = s.length - 1; i >= 0; i -= 1) {
    switch (s.charAt(i)) {
      case 'I':
        num = 1
        break
      case 'V':
        num = 5
        break
      case 'X':
        num = 10
        break
      case 'L':
        num = 50
        break
      case 'C':
        num = 100
        break
      case 'D':
        num = 500
        break
      case 'M':
        num = 1000
        break
    }

    const temp = num * 4
    result += temp < result ? -num : num
  }

  return result
}

interface Input {
  readonly s: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      s: 'III'
    },
    {
      s: 'LVIII'
    },
    {
      s: 'MCMXCIV'
    }
  ]

  for (const { s } of inputs) {
    const result = romanToInt(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
