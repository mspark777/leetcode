/**
 Do not return anything, modify s in-place instead.
 */
function reverseString(s: string[]): void {
  let i = 0;
  let j = s.length - 1
  while (i < j) {
    [s[i], s[j]] = [s[j], s[i]]
    i += 1
    j -= 1
  }
}

async function main(): Promise<void> {
  const inputs: string[][] = [
    ["h", "e", "l", "l", "o"],
    ["H", "a", "n", "n", "a", "h"]
  ]

  for (const s of inputs) {
    reverseString(s)
    console.log(s)
  }
}

await main()
