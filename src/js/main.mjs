/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/no-extraneous-class */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unnecessary-condition */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

class ListNode {
  constructor(val, next) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

class TreeNode {
  constructor(val, left, right) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}

/**
 * @param {ListNode | undefined} node
 * @param {number} patternindex
 * @param {number[]} pattern
 * @param {number[]} prefixTable
 * @return {boolean}
 */
function searchInTree(node, patternIndex, pattern, prefixTable) {
  if (node == null) {
    return false;
  }

  while (patternIndex > 0 && node.val !== pattern[patternIndex]) {
    patternIndex = prefixTable[patternIndex - 1];
  }

  if (node.val === pattern[patternIndex]) {
    patternIndex += 1;
  }

  if (patternIndex === pattern.length) {
    return true;
  }

  return (
    searchInTree(node.left, patternIndex, pattern, prefixTable) ||
    searchInTree(node.right, patternIndex, pattern, prefixTable)
  );
}

/**
 * @param {ListNode | undefined} head
 * @param {TreeNode | undefined} root
 * @return {boolean}
 */
function isSubPath(head, root) {
  if (head == null) {
    return false;
  } else if (root == null) {
    return false;
  }

  const pattern = [head.val];
  const prefixTable = [0];
  let patternIndex = 0;
  head = head.next;

  while (head != null) {
    while (patternIndex > 0 && head.val !== pattern[patternIndex]) {
      patternIndex = prefixTable[patternIndex - 1];
    }
    if (head.val === pattern[patternIndex]) {
      patternIndex += 1;
    }

    pattern.push(head.val);
    prefixTable.push(patternIndex);
    head = head.next;
  }

  return searchInTree(root, 0, pattern, prefixTable);
}

function main() {
  const inputs = [
    [
      [0, 1, 1, 0],
      [0, 1, 1, 0],
      [0, 0, 0, 0],
    ],
    [[1, 1]],
  ];

  for (const input of inputs) {
    const result = minDays(input);
    console.log(result);
  }
}
main();
