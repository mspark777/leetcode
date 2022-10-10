function breakPalindrome (palindrome: string): string {
  if (palindrome.length <= 1) {
    return ''
  }

  const chars = palindrome.split('')
  for (let i = 0; i < Math.floor(chars.length / 2); i += 1) {
    if (chars[i] !== 'a') {
      chars[i] = 'a'
      return chars.join('')
    }
  }

  chars[chars.length - 1] = 'b'
  return chars.join('')
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'abccba',
    'a',
    'aba'
  ]

  for (const palindrome of inputs) {
    const result = breakPalindrome(palindrome)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
