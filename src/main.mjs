function transform (s) {
  const result = new Array(s.length)
  const indexMapping = new Map()
  for (let i = 0; i < s.length; i += 1) {
    const ch = s.charAt(i)

    if (!indexMapping.has(ch)) {
      indexMapping.set(ch, i)
    }

    const idx = indexMapping.get(ch)
    result[i] = idx
  }

  return result.join(' ')
}

/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
function isIsomorphic (s, t) {
  return transform(s) === transform(t)
}

async function main () {
  const inputs = [
    {
      s: 'egg',
      t: 'add'
    },
    {
      s: 'foo',
      t: 'bar'
    },
    {
      s: 'paper',
      t: 'title'
    }
  ]

  for (const { s, t } of inputs) {
    const result = isIsomorphic(s, t)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
