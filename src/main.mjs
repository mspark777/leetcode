/**
  * @param {string[]} letters
  * @param {string} target
  * @returns {string}
  */
function nextGreatestLetter (letters, target) {
  let left = 0
  let right = letters.length - 1
  while (left <= right) {
    const middle = Math.trunc((left + right) / 2)
    const letter = letters[middle]
    const cmp = letter.localeCompare(target)
    if (cmp <= 0) {
      left = middle + 1
    } else {
      right = middle - 1
    }
  }

  return left < letters.length ? letters[left] : letters[0]
}

function main () {
  const inputs = [
    [['c', 'f', 'j'], 'a'],
    [['c', 'f', 'j'], 'c'],
    [['x', 'x', 'y', 'y'], 'z']
  ]

  for (const [letters, target] of inputs) {
    const result = nextGreatestLetter(letters, target)
    console.log(result)
  }
}
main()
