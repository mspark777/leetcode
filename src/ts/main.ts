import { UnionFind } from './utils'

function minScore (n: number, roads: number[][]): number {
  const uf = new UnionFind(n + 1)
  let result = Number.MAX_SAFE_INTEGER

  for (const [a, b] of roads) {
    uf.union(a, b)
  }

  for (const road of roads) {
    const a = road[0]
    const d = road[2]
    if (uf.find(1) === uf.find(a)) {
      result = Math.min(result, d)
    }
  }

  return result
}

interface Input {
  readonly n: number
  readonly roads: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: 4, roads: [[1, 2, 9], [2, 3, 6], [2, 4, 5], [1, 4, 7]] },
    { n: 4, roads: [[1, 2, 2], [1, 3, 4], [3, 4, 7]] }
  ]

  for (const { n, roads } of inputs) {
    const result = minScore(n, roads)
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

/*
  *
  *
class Solution {
    public int minScore(int n, int[][] roads) {
        UnionFind dsu = new UnionFind(n + 1);
        int answer = Integer.MAX_VALUE;

        for (int[] road : roads) {
            dsu.union_set(road[0], road[1]);
        }

        for (int[] road : roads) {
            if (dsu.find(1) == dsu.find(road[0])) {
                answer = Math.min(answer, road[2]);
            }
        }

        return answer;
    }
}
  */
