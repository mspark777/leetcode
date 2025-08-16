import "@total-typescript/ts-reset";

function busyStudent(
  startTime: number[],
  endTime: number[],
  queryTime: number,
): number {
  let result = 0;

  for (const [i, s] of startTime.entries()) {
    const e = endTime[i];
    if (e == null) {
      continue;
    } else if (s <= queryTime && queryTime <= e) {
      result += 1;
    }
  }

  return result;
}

interface Input {
  startTime: number[];
  endTime: number[];
  queryTime: number;
}

function main(): void {
  const inputs: Input[] = [
    {
      startTime: [1, 2, 3],
      endTime: [3, 2, 7],
      queryTime: 4,
    },
    {
      startTime: [4],
      endTime: [4],
      queryTime: 4,
    },
  ];

  for (const input of inputs) {
    const result = busyStudent(input.startTime, input.endTime, input.queryTime);
    console.log(result);
  }
}
main();
