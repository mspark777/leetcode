const { newList, ListNode } = require('./lib')

function unused () {}
unused(ListNode)

class Solution {
  /** @type {ListNode|null} */
  #head
  constructor (head) {
    this.#head = head
  }

  /**
    * @returns {number}
    */
  getRandom () {
    let scope = 1
    let result = 0
    for (let curr = this.#head; curr != null; curr = curr?.next) {
      if ((Math.random() * scope) < 1.0) {
        result = curr.val
      }

      scope += 1
    }

    return result
  }
}

async function main () {
  const solution = new Solution(newList([1, 2, 3]))
  console.log(solution.getRandom())
  console.log(solution.getRandom())
  console.log(solution.getRandom())
  console.log(solution.getRandom())
  console.log(solution.getRandom())
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
