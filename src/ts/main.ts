import "@total-typescript/ts-reset";

function minCostToMoveChips(position: number[]): number {
  let odd = 0;
  let even = 0;
  for (const p of position) {
    if (p % 2 === 0) {
      even += 1;
    } else {
      odd += 1;
    }
  }

  return Math.min(even, odd);
}

interface Input {
  position: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      position: [1, 2, 3],
    },
    {
      position: [2, 2, 2, 3, 3],
    },
    {
      position: [1, 1000000000],
    },
  ];

  for (const input of inputs) {
    const result = minCostToMoveChips(input.position);
    console.log(result);
  }
}
main();
