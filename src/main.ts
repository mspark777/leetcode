function find (parents: Map<string, string>, code: string): string {
  let parent = parents.get(code) as string
  if (parent === code) {
    return code
  }

  parent = find(parents, parent)
  parents.set(code, parent)
  return parent
}

function union (parents: Map<string, string>, a: string, b: string): void {
  const parenta = find(parents, a)
  const parentb = find(parents, b)
  parents.set(parentb, parenta)
}

function equationsPossible (equations: string[]): boolean {
  const parents = new Map<string, string>()
  for (let i = 0; i < 26; i += 1) {
    const key = String.fromCharCode('a'.charCodeAt(0) + i)
    parents.set(key, key)
  }

  for (const equation of equations) {
    if (equation[1] === '=') {
      const a = parents.get(equation[0]) as string
      const b = parents.get(equation[3]) as string
      if (find(parents, a) !== find(parents, b)) {
        union(parents, a, b)
      }
    }
  }

  for (const equation of equations) {
    if (equation[1] === '!') {
      const a = parents.get(equation[0]) as string
      const b = parents.get(equation[3]) as string
      if (find(parents, a) === find(parents, b)) {
        return false
      }
    }
  }

  return true
}

async function main (): Promise<void> {
  const inputs: string[][] = [
    ['a==b', 'b!=a'],
    ['b==a', 'a==b'],
    ['c==c', 'b==d', 'x!=z']
  ]

  for (const input of inputs) {
    const result = equationsPossible(input)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
