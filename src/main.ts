function minMutation (start: string, end: string, bank: string[]): number {
  const bankSet = new Set(bank)
  const seens = new Set([start])
  let queue = [start]

  let result = 0

  while (queue.length > 0) {
    const queueLen = queue.length
    for (let i = 0; i < queueLen; i += 1) {
      const gene = queue[i]

      if (gene === end) {
        return result
      }

      for (const g of 'ACGT') {
        for (let j = 0; j < gene.length; j += 1) {
          const genes = [...gene]
          genes[j] = g
          const neighbor = genes.join('')

          if (!seens.has(neighbor) && bankSet.has(neighbor)) {
            queue.push(neighbor)
            seens.add(neighbor)
          }
        }
      }
    }

    result += 1
    queue = queue.slice(queueLen)
  }

  return -1
}

interface Input {
  readonly start: string
  readonly end: string
  readonly bank: string[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { start: 'AACCGGTT', end: 'AACCGGTA', bank: ['AACCGGTA'] },
    { start: 'AACCGGTT', end: 'AAACGGTA', bank: ['AACCGGTA', 'AACCGCTA', 'AAACGGTA'] },
    { start: 'AAAAACCC', end: 'AACCCCCC', bank: ['AAAACCCC', 'AAACCCCC', 'AACCCCCC'] },
    { start: 'AACCGGTT', end: 'AACCGCTA', bank: ['AACCGGTA', 'AACCGCTA', 'AAACGGTA'] }
  ]

  for (const { start, end, bank } of inputs) {
    const result = minMutation(start, end, bank)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
