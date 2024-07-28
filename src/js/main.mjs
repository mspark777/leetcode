/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number} n
 * @param {number[][]} edges
 * @param {number} time
 * @param {number} change
 * @return {number}
 */
function secondMinimum(n, edges, time, change) {
  const adjacencyMap = new Map();
  for (const [a, b] of edges) {
    const aList = adjacencyMap.get(a) ?? [];
    const bList = adjacencyMap.get(b) ?? [];

    aList.push(b);
    bList.push(a);

    adjacencyMap.set(a, aList);
    adjacencyMap.set(b, bList);
  }

  const dist1 = [-1];
  const dist2 = [-1];
  for (let i = 0; i < n; i += 1) {
    dist1.push(-1);
    dist2.push(-1);
  }

  const queue = [[1, 1]];
  dist1[1] = 0;

  while (queue.length > 0) {
    const [node, freq] = queue.shift();
    let timeTaken = freq === 1 ? dist1[node] : dist2[node];
    if ((BigInt(timeTaken) / BigInt(change)) % 2n === 1n) {
      timeTaken =
        change * Number(BigInt(timeTaken) / BigInt(change) + 1n) + time;
    } else {
      timeTaken = timeTaken + time;
    }

    for (const neighbor of adjacencyMap.get(node) ?? []) {
      if (dist1[neighbor] === -1) {
        dist1[neighbor] = timeTaken;
        queue.push([neighbor, 1]);
      } else if (dist2[neighbor] === -1 && dist1[neighbor] !== timeTaken) {
        if (neighbor === n) {
          return timeTaken;
        }

        dist2[neighbor] = timeTaken;
        queue.push([neighbor, 2]);
      }
    }
  }

  return 0;
}

function main() {
  const inputs = [
    [
      [3, 8],
      [4, 7],
    ],
    [
      [5, 7, 10],
      [8, 6, 8],
    ],
  ];

  for (const [rowSum, colSum] of inputs) {
    const result = restoreMatrix(rowSum, colSum);
    console.log(result);
  }
}
main();
