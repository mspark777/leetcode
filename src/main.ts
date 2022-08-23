class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function isPalindrome (head: ListNode | null): boolean {
  const nums: number[] = []
  while (head != null) {
    nums.push(head.val)
    head = head.next
  }

  for (let i = 0, j = nums.length - 1; i < j; i += 1, j -= 1) {
    if (nums[i] !== nums[j]) {
      return false
    }
  }

  return true
}

interface Input {
  readonly nums: number[]
}

function arrToList (nums: number[]): ListNode | null {
  const dummy = new ListNode()
  let tail = dummy
  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return dummy.next
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 2, 2, 1]
    },
    {
      nums: [1, 2]
    }
  ]

  for (const input of inputs) {
    const nums = input.nums
    const result = isPalindrome(arrToList(nums))
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
