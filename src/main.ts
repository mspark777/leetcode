function groupAnagrams (strs: string[]): string[][] {
  const map = new Map<string, string[]>()

  for (const str of strs) {
    const key = [...str].sort().join('')
    const group = map.get(key)
    if (group != null) {
      group.push(str)
    } else {
      map.set(key, [str])
    }
  }

  return Array.from(map.values())
}

async function main (): Promise<void> {
  const inputs: string[][] = [
    ['eat', 'tea', 'tan', 'ate', 'nat', 'bat'],
    [''],
    ['a']
  ]

  for (const strs of inputs) {
    const result = groupAnagrams(strs)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
