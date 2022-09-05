function decodeString (s: string): string {
  const stack: string[] = []

  for (const ch of s) {
    if (ch !== ']') {
      stack.push(ch)
      continue
    }

    const chars: string[] = []
    for (let top = stack.pop(); top !== '['; top = stack.pop()) {
      chars.push(top as string)
    }

    const nums: string[] = []
    while (stack.length > 0) {
      const nch = stack.pop() as string
      const n = Number(nch)
      if (Number.isNaN(n)) {
        stack.push(nch)
        break
      } else {
        nums.push(nch)
      }
    }

    const str = chars.reverse().join('')
    const count = Number(nums.reverse().join(''))

    for (let i = 0; i < count; i += 1) {
      stack.push(str)
    }
  }

  return stack.join('')
}

interface Input {
  readonly s: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      s: '3[a]2[bc]'
    },
    {
      s: '3[a2[c]]'
    },
    {
      s: '2[abc]3[cd]ef'
    }
  ]

  for (const { s } of inputs) {
    const result = decodeString(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
