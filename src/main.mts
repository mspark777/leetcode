function minDeletionSize(strs: string[]): number {
  let result = 0
  for (let c = 0; c < strs[0].length; c += 1) {
    for (let r = 1; r < strs.length; r += 1) {
      const c0 = strs[r - 1].charCodeAt(c)
      const c1 = strs[r].charCodeAt(c)
      if (c0 > c1) {
        result += 1
        break
      }
    }
  }

  return result
}

async function main(): Promise<void> {
  const inputs: string[][] = [
    ["cba", "daf", "ghi"],
    ["a", "b"],
    ["zyx", "wvu", "tsr"]
  ]

  for (const strs of inputs) {
    const result = minDeletionSize(strs)
    console.log(result)
  }
}

await main()
