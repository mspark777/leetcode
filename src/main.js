/**
 * @param {string} s1
 * @param {string} s2
 * @returns {boolean}
 */
function checkInclusion (s1, s2) {
  if (s1.length > s2.length) {
    return false
  }

  const LETTERS = 26
  /** @type Array<number> */
  const s1map = new Array(LETTERS).fill(0)
  /** @type Array<number> */
  const s2map = new Array(LETTERS).fill(0)

  const ACODE = 'a'.charCodeAt(0)
  for (let i = 0; i < s1.length; i += 1) {
    const code1 = s1.charCodeAt(i) - ACODE
    const code2 = s2.charCodeAt(i) - ACODE

    s1map[code1] += 1
    s2map[code2] += 1
  }

  let count = 0
  for (let i = 0; i < LETTERS; i += 1) {
    if (s1map[i] === s2map[i]) {
      count += 1
    }
  }

  for (let i = 0; i < s2.length - s1.length; i += 1) {
    if (count === LETTERS) {
      break
    }

    const left = s2.charCodeAt(i) - ACODE
    s2map[left] -= 1
    if (s1map[left] === s2map[left]) {
      count += 1
    } else if ((s1map[left] - 1) === s2map[left]) {
      count -= 1
    }

    const right = s2.charCodeAt(i + s1.length) - ACODE
    s2map[right] += 1
    if (s1map[right] === s2map[right]) {
      count += 1
    } else if ((s1map[right] + 1) === s2map[right]) {
      count -= 1
    }
  }

  return count === LETTERS
}

async function main () {
  const inputs = [
    ['ab', 'eidbaooo'],
    ['ab', 'eidboaoo']
  ]

  for (const [s1, s2] of inputs) {
    const result = checkInclusion(s1, s2)
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
