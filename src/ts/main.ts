import "@total-typescript/ts-reset";

function getConcatenation(nums: number[]): number[] {
  return [...nums, ...nums];
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 2, 1],
    },
    {
      nums: [1, 3, 2, 1],
    },
  ];

  for (const input of inputs) {
    const result = getConcatenation(input.nums);
    console.log(result);
  }
}
main();
