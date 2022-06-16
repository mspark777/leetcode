function longestPalindrome (s: string): string {
  if (s.length < 2) {
    return s
  } else if (s.length === 2) {
    return (s.charCodeAt(0) === s.charCodeAt(1))
      ? s
      : s.substring(0, 1)
  }

  const strLen = s.length
  let maxLength = 0
  let start = 0
  for (let i = 1; i < (strLen - 1); i += 1) {
    const code = s.charCodeAt(i)
    let low = i - 1
    let high = i + 1

    while (low > -1) {
      if (s.charCodeAt(low) === code) {
        low -= 1
      } else {
        break
      }
    }

    while (high < strLen) {
      if (s.charCodeAt(high) === code) {
        high += 1
      } else {
        break
      }
    }

    while ((low > -1) && (high < strLen)) {
      if (s.charCodeAt(low) === s.charCodeAt(high)) {
        low -= 1
        high += 1
      } else {
        break
      }
    }

    const length = high - low - 1
    if (maxLength < length) {
      maxLength = length
      start = low + 1
    }
  }

  return s.substring(start, maxLength + start)
}

async function main (): Promise<void> {
  const inputs = [
    'babad', 'cbbd', 'ac', 'aba', 'bb',
    'zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz'
  ]

  for (const input of inputs) {
    const result = longestPalindrome(input)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
