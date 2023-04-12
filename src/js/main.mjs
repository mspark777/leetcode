/**
  * @param {string} path
  * @returns {string}
  */
function simplifyPath (path) {
  const segments = path.split('/')
  const result = []

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

async function main () {
  const inputs = [
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
