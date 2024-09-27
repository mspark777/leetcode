/* eslint-disable @typescript-eslint/explicit-function-return-type */

class MyCalendarTwo {
  /** @type {Map<number, number>} */
  bookingCount;

  constructor() {
    this.bookingCount = new Map();
  }

  /**
   * @param {number} start
   * @param {number} end
   * @return {boolean}
   */
  book(start, end) {
    this.inc(start, 1);
    this.inc(end, -1);

    let overlappedBooking = 0;
    const values = [...this.bookingCount]
      .sort((l, r) => l[0] - r[0])
      .map((l) => l[1]);

    for (const count of values) {
      overlappedBooking += count;
      if (overlappedBooking <= 2) {
        continue;
      }

      this.inc(start, -1);
      this.inc(end, 1);

      if (this.get(start) === 0) {
        this.bookingCount.delete(start);
      }

      if (this.get(end) === 0) {
        this.bookingCount.delete(end);
      }

      return false;
    }

    return true;
  }

  /**
   * @param {number} key
   * @param {number} value
   * @return {number}
   */
  inc(key, value) {
    const count = this.bookingCount.get(key);
    if (count == null) {
      this.bookingCount.set(key, value);
      return value;
    }

    const cnt = count + value;
    this.bookingCount.set(key, cnt);
    return cnt;
  }
  /**
   * @param {number} n
   * @return {number | undefined}
   */
  get(n) {
    return this.bookingCount.get(n);
  }
}

function main() {
  const inputs = ["2-1-1", "2*3-4*5"];

  for (const expression of inputs) {
    const result = diffWaysToCompute(expression);
    console.log(result);
  }
}
main();
