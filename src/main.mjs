/**
 * @param {Map<string, string>} parents
 * @param {string} code
 * @returns {string}
 */
function find (parents, code) {
  let parent = parents.get(code)
  if (parent === code) {
    return code
  }

  parent = find(parents, parent)
  parents.set(code, parent)
  return parent
}

/**
 * @param {Map<string, string>} parents
 * @param {string} a
 * @param {string} b
 * @returns {void}
 */
function union (parents, a, b) {
  const parenta = find(parents, a)
  const parentb = find(parents, b)
  parents.set(parentb, parenta)
}

/**
 * @param {string[]} equations
 * @returns {boolean}
 */
function equationsPossible (equations) {
  const parents = new Map()
  for (let i = 0; i < 26; i += 1) {
    const key = String.fromCharCode('a'.charCodeAt(0) + i)
    parents.set(key, key)
  }

  for (const equation of equations) {
    if (equation[1] === '=') {
      const a = parents.get(equation[0])
      const b = parents.get(equation[3])
      if (find(parents, a) !== find(parents, b)) {
        union(parents, a, b)
      }
    }
  }

  for (const equation of equations) {
    if (equation[1] === '!') {
      const a = parents.get(equation[0])
      const b = parents.get(equation[3])
      if (find(parents, a) === find(parents, b)) {
        return false
      }
    }
  }

  return true
}

async function main () {
  const inputs = [
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
