import '@total-typescript/ts-reset'

function letterCombinations (digits: string): string[] {
  if (digits.length < 1) {
    return []
  }

  const lettersMap = new Map<string, string>()
  lettersMap.set('2', 'abc')
  lettersMap.set('3', 'def')
  lettersMap.set('4', 'ghi')
  lettersMap.set('5', 'jkl')
  lettersMap.set('6', 'mno')
  lettersMap.set('7', 'pqrs')
  lettersMap.set('8', 'tuv')
  lettersMap.set('9', 'wxyz')

  const stack: string[] = ['']
  const result: string[] = []

  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const toplen = top.length
    const ch = digits[toplen] as string
    const letters = lettersMap.get(ch) ?? ''
    for (const letter of letters) {
      const combination = top + letter
      if (combination.length === digits.length) {
        result.push(combination)
      } else {
        stack.push(combination)
      }
    }
  }

  return result
}

function main (): void {
  const inputs: string[] = [
    '23',
    '',
    '2'
  ]

  for (const digits of inputs) {
    const result = letterCombinations(digits)
    console.log(result)
  }
}
main()
