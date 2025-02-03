import "@total-typescript/ts-reset";

function numUniqueEmails(emails: string[]): number {
  const emailSet = new Set<string>();

  for (const email of emails) {
    const chunks = email.split("@");
    let local = chunks[0] as string;
    const plus = local.indexOf("+");
    if (plus >= 0) {
      local = local.slice(0, plus);
    }
    local = local.replaceAll(".", "");
    const domain = chunks[1] as string;
    emailSet.add(`${local}@${domain}`);
  }

  return emailSet.size;
}

function main(): void {
  const inputs: Array<string[]> = [
    [
      "test.email+alex@leetcode.com",
      "test.e.mail+bob.cathy@leetcode.com",
      "testemail+david@lee.tcode.com",
    ],
    ["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"],
  ];

  for (const email of inputs) {
    const result = numUniqueEmails(email);
    console.log(result);
  }
}
main();
