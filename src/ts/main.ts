import { newList, type ListNode } from './lib'

class Solution {
  private readonly head: ListNode | null
  public constructor (head: ListNode | null) {
    this.head = head
  }

  public getRandom (): number {
    let scope = 1
    let result = 0
    for (let curr = this.head; curr != null; curr = curr?.next) {
      if ((Math.random() * scope) < 1.0) {
        result = curr.val
      }

      scope += 1
    }

    return result
  }
}

async function main (): Promise<void> {
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
