class ListNode {
  /** @type {number} */
  val
  /** @type {ListNode|null} */
  next

  /**
    * @param {number} [val]
    * @param {ListNode|null} [next]
    */
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

/**
  * @param {ListNode|null} node
  * @returns {number[]}
  */
function ltoa (node) {
  const arr = []
  while (node != null) {
    arr.push(node.val)
    node = node.next
  }

  return arr
}

/**
  * @param {number[]} arr
  * @returns {boolean}
  */
function isNotEmpty (arr) {
  return arr.length > 0
}

/**
  * @param {ListNode|null} l1
  * @param {ListNode|null} l2
  * @returns {ListNode|null}
  */
function addTwoNumbers (l1, l2) {
  const stack1 = ltoa(l1)
  const stack2 = ltoa(l2)

  let total = 0n
  let carry = 0n
  let result = new ListNode()
  while (isNotEmpty(stack1) || isNotEmpty(stack2)) {
    if (isNotEmpty(stack1)) {
      total += BigInt(stack1.pop())
    }

    if (isNotEmpty(stack2)) {
      total += BigInt(stack2.pop())
    }

    result.val = Number(total % 10n)
    carry = total / 10n
    const head = new ListNode(Number(carry))
    head.next = result
    result = head
    total = carry
  }

  return carry === 0n ? result.next : result
}

/**
  * @param {number[]} arr
  * @returns {ListNode|null}
  */
function atol (arr) {
  const head = new ListNode()
  let tail = head
  for (const num of arr) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return head.next
}

function main () {
  const inputs = [
    [atol([7, 2, 4, 3]), atol([5, 6, 4])],
    [atol([2, 4, 3]), atol([5, 6, 4])],
    [atol([0]), atol([0])]
  ]

  for (const [l1, l2] of inputs) {
    const result = addTwoNumbers(l1, l2)
    console.log(ltoa(result))
  }
}
main()
