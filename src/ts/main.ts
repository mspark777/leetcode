import "@total-typescript/ts-reset";

function div(l: number, r: number): number {
  return Number(BigInt(l) / BigInt(r));
}

function findEvenNumbers(digits: number[]): number[] {
  const frequencies = new Map<number, number>();
  const result: number[] = [];

  for (const digit of digits) {
    frequencies.set(digit, (frequencies.get(digit) ?? 0) + 1);
  }

  for (let i = 100; i < 1000; i += 2) {
    const unit = i % 10;
    const ten = div(i, 10) % 10;
    const hundred = div(i, 100);

    frequencies.set(unit, (frequencies.get(unit) ?? 0) - 1);
    frequencies.set(ten, (frequencies.get(ten) ?? 0) - 1);
    frequencies.set(hundred, (frequencies.get(hundred) ?? 0) - 1);

    const unit_frequency = frequencies.get(unit) as number;
    const ten_frequency = frequencies.get(ten) as number;
    const hundred_frequency = frequencies.get(hundred) as number;

    const ok = [unit_frequency, ten_frequency, hundred_frequency].every(
      f => f >= 0,
    );

    if (ok) {
      result.push(i);
    }

    frequencies.set(unit, (frequencies.get(unit) ?? 0) + 1);
    frequencies.set(ten, (frequencies.get(ten) ?? 0) + 1);
    frequencies.set(hundred, (frequencies.get(hundred) ?? 0) + 1);
  }

  return result;
}

interface Input {
  digits: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      digits: [2, 1, 3, 0],
    },
    {
      digits: [2, 2, 8, 8, 2],
    },
    {
      digits: [3, 7, 5],
    },
  ];

  for (const input of inputs) {
    const result = findEvenNumbers(input.digits);
    console.log(result);
  }
}
main();
