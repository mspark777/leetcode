/**
  * @param {string} senate
  * @returns {string}
  */
function predictPartyVictory (senate) {
  const R = 'R'
  const D = 'D'
  let rcount = 0
  let dcount = 0
  let dfloating = 0
  let rfloating = 0

  const queue = senate.split('')
  for (const c of queue) {
    if (c === R) {
      rcount += 1
    } else {
      dcount += 1
    }
  }

  while ((rcount > 0) && (dcount > 0)) {
    const curr = queue.shift()

    if (curr === D) {
      if (dfloating > 0) {
        dfloating -= 1
        dcount -= 1
      } else {
        rfloating += 1
        queue.push(D)
      }
    } else {
      if (rfloating > 0) {
        rfloating -= 1
        rcount -= 1
      } else {
        dfloating += 1
        queue.push(R)
      }
    }
  }

  return rcount > 0 ? 'Radiant' : 'Dire'
}

async function main () {
  const inputs = [
    'RD',
    'RDD'
  ]

  for (const senate of inputs) {
    const result = predictPartyVictory(senate)
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
