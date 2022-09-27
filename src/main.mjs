/**
 * @param {string} dominoes
 * @returns {string}
 */
function pushDominoes (dominoes) {
  const LEFT = 'L'
  const RIGHT = 'R'
  const STAND = '.'
  const forces = new Array(dominoes.length).fill(0)

  let force = 0
  for (let i = 0; i < dominoes.length; i += 1) {
    const ch = dominoes[i]
    if (ch === LEFT) {
      force = 0
    } else if (ch === RIGHT) {
      force = dominoes.length
    } else {
      force = Math.max(force - 1, 0)
    }

    forces[i] += force
  }

  force = 0
  for (let i = dominoes.length - 1; i >= 0; i -= 1) {
    const ch = dominoes[i]
    if (ch === LEFT) {
      force = dominoes.length
    } else if (ch === RIGHT) {
      force = 0
    } else {
      force = Math.max(force - 1, 0)
    }

    forces[i] -= force
  }

  const result = new Array(dominoes.length)
  for (let i = 0; i < dominoes.length; i += 1) {
    const force = forces[i]
    if (force < 0) {
      result[i] = LEFT
    } else if (force > 0) {
      result[i] = RIGHT
    } else {
      result[i] = STAND
    }
  }

  return result.join('')
}

async function main () {
  const inputs = [
    'RR.L',
    '.L.R...LR..L..'
  ]

  for (const input of inputs) {
    const result = pushDominoes(input)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
