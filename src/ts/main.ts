import "@total-typescript/ts-reset";

function decompressRLElist(nums: number[]): number[] {
  const result: number[] = [];

  for (let i = 0; i < nums.length; i += 2) {
    const frequency = nums[i];
    const value = nums[i + 1];

    if (frequency == null) {
      continue;
    } else if (value == null) {
      continue;
    }

    for (let j = 0; j < frequency; j += 1) {
      result.push(value);
    }
  }

  return result;
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 2, 3, 4],
    },
    {
      nums: [1, 1, 2, 3],
    },
  ];

  for (const input of inputs) {
    const result = decompressRLElist(input.nums);
    console.log(result);
  }
}
main();
