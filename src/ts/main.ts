import "@total-typescript/ts-reset";

function countMatches(
  items: string[][],
  ruleKey: string,
  ruleValue: string,
): number {
  return items.reduce((acc, [type, color, name]) => {
    if (ruleKey === "type" && ruleValue === type) {
      return acc + 1;
    } else if (ruleKey === "color" && ruleValue === color) {
      return acc + 1;
    } else if (ruleKey === "name" && ruleValue === name) {
      return acc + 1;
    }
    return acc;
  }, 0);
}

interface Input {
  items: string[][];
  ruleKey: string;
  ruleValue: string;
}

function main(): void {
  const inputs: Input[] = [
    {
      items: [
        ["phone", "blue", "pixel"],
        ["computer", "silver", "lenovo"],
        ["phone", "gold", "iphone"],
      ],
      ruleKey: "color",
      ruleValue: "silver",
    },
    {
      items: [
        ["phone", "blue", "pixel"],
        ["computer", "silver", "phone"],
        ["phone", "gold", "iphone"],
      ],
      ruleKey: "type",
      ruleValue: "phone",
    },
  ];

  for (const input of inputs) {
    const result = countMatches(input.items, input.ruleKey, input.ruleValue);
    console.log(result);
  }
}
main();
