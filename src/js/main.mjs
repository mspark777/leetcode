import { UnionFind } from './utils.mjs'

/**
  * @param {string} a
  * @param {string} b
  * @returns {boolean}
  */
function isSimilar (a, b) {
  let diff = 0

  for (let i = 0; i < a.length; i += 1) {
    if (a.charAt(i) !== b.charAt(i)) {
      diff += 1
    }
  }

  return [0, 2].includes(diff)
}

/**
  * @param {string[]} strs
  * @returns {number}
  */
function numSimilarGroups (strs) {
  const uf = new UnionFind(strs.length)
  let result = strs.length
  for (let i = 0; i < strs.length; i += 1) {
    const a = strs[i]
    for (let j = i + 1; j < strs.length; j += 1) {
      const b = strs[j]
      if (!isSimilar(a, b)) {
        continue
      }

      if (uf.find(i) !== uf.find(j)) {
        result -= 1
        uf.union(i, j)
      }
    }
  }

  return result
}

async function main () {
  const inputs = [
    ['tars', 'rats', 'arts', 'star'],
    ['omv', 'ovm']
  ]

  for (const strs of inputs) {
    const result = numSimilarGroups(strs)
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
