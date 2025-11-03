import "@total-typescript/ts-reset";

function averageValue(nums: number[]): number {
  const filtered = nums
    .filter((n) => BigInt(n) % 2n === 0n)
    .filter((n) => BigInt(n) % 3n === 0n);
  if (filtered.length < 1) {
    return 0;
  }

  const avg = filtered.reduce((acc, num) => acc + num) / filtered.length;

  return Math.floor(avg);
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 3, 6, 10, 12, 15],
    },
    {
      nums: [1, 2, 4, 7, 10],
    },
    {
      nums: [4, 4, 9, 10],
    },
  ];

  for (const input of inputs) {
    const result = averageValue(input.nums);
    console.log(result);
  }
}
main();
