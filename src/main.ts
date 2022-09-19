function findDuplicate (paths: string[]): string[][] {
  const pathMap = new Map<string, string[]>()
  for (const path of paths) {
    const segments = path.split(' ')
    const root = segments[0]
    for (let i = 1; i < segments.length; i += 1) {
      const file = segments[i]
      const sep = file.indexOf('(')
      const name = file.substring(0, sep)
      const content = file.substring(sep)
      const list = pathMap.get(content) ?? []
      list.push(`${root}/${name}`)
      pathMap.set(content, list)
    }
  }

  const result: string[][] = []
  for (const value of pathMap.values()) {
    if (value.length > 1) {
      result.push(value)
    }
  }

  return result
}

interface Input {
  readonly paths: string[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      paths: ['root/a 1.txt(abcd) 2.txt(efgh)', 'root/c 3.txt(abcd)', 'root/c/d 4.txt(efgh)', 'root 4.txt(efgh)']
    },
    {
      paths: ['root/a 1.txt(abcd) 2.txt(efgh)', 'root/c 3.txt(abcd)', 'root/c/d 4.txt(efgh)']
    },
    {
      paths: ['root/a 1.txt(abcd) 2.txt(efsfgh)', 'root/c 3.txt(abdfcd)', 'root/c/d 4.txt(efggdfh)']
    }
  ]

  for (const { paths } of inputs) {
    const result = findDuplicate(paths)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
