function strStr (haystack: string, needle: string): number {
  const m = needle.length
  const n = haystack.length

  for (let start = 0; start <= n - m; start += 1) {
    for (let i = 0; i < m; i += 1) {
      if (needle[i] !== haystack[start + i]) {
        break
      }

      if (i === (m - 1)) {
        return start
      }
    }
  }

  return -1
}

async function main (): Promise<void> {
  const inputs: string[][] = [
    ['sadbutsad', 'sad'],
    ['leetcode', 'leeto']
  ]

  for (const [haystack, needle] of inputs) {
    const result = strStr(haystack, needle)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
