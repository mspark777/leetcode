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
 * @return {number[][]}
 */
function getAncestors(n, edges) {
  const adjacencyList = [];
  const indegrees = [];
  const ancestorsSetList = [];
  for (let i = 0; i < n; i += 1) {
    adjacencyList.push([]);
    indegrees.push(0);
    ancestorsSetList.push(new Set());
  }

  for (const [from, to] of edges) {
    adjacencyList[from].push(to);
    indegrees[to] += 1;
  }

  const nodesWithZeroIndegree = [];
  for (const [i, indegree] of indegrees.entries()) {
    if (indegree === 0) {
      nodesWithZeroIndegree.push(i);
    }
  }

  const topologicalOrder = [];
  while (nodesWithZeroIndegree.length > 0) {
    const currentNode = nodesWithZeroIndegree.shift();
    topologicalOrder.push(currentNode);

    for (const neighbor of adjacencyList[currentNode]) {
      indegrees[neighbor] -= 1;
      if (indegrees[neighbor] === 0) {
        nodesWithZeroIndegree.push(neighbor);
      }
    }
  }

  const result = [];
  for (const node of topologicalOrder) {
    for (const neighbor of adjacencyList[node]) {
      ancestorsSetList[neighbor].add(node);
      for (const n of ancestorsSetList[node]) {
        ancestorsSetList[neighbor].add(n);
      }
    }
  }
  for (let i = 0; i < n; i += 1) {
    const list = [...ancestorsSetList[i]];
    list.sort((l, r) => l - r);
    result.push(list);
  }

  return result;
}

function main() {
  const inputs = [
    [
      [
        [0, 3],
        [0, 4],
        [1, 3],
        [2, 4],
        [2, 7],
        [3, 5],
        [3, 6],
        [3, 7],
        [4, 6],
      ],
      8,
    ],
    [
      [
        [0, 1],
        [0, 2],
        [0, 3],
        [0, 4],
        [1, 2],
        [1, 3],
        [1, 4],
        [2, 3],
        [2, 4],
        [3, 4],
      ],
      5,
    ],
  ];

  for (const [edges, n] of inputs) {
    const result = getAncestors(n, edges);
    console.log(result);
  }
}
main();
