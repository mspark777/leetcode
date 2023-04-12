function simplifyPath (path: string): string {
  const segments = path.split('/')
  const result: string[] = []

  for (const seg of segments) {
    if (seg === '') {
      continue
    } else if (seg === '.') {
      continue
    } else if (seg === '..') {
      result.pop()
    } else {
      result.push(seg)
    }
  }
  return `/${result.join('/')}`
}

async function main (): Promise<void> {
  const inputs: string[] = [
    '/home/',
    '/../',
    '/home//foo/'
  ]

  for (const path of inputs) {
    const result = simplifyPath(path)
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
