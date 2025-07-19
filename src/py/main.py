from typing import List, Callable
from threading import Lock
import math


class Foo:
    first_lock: Lock
    second_lock: Lock

    def __init__(self):
        self.first_lock = Lock()
        self.second_lock = Lock()
        self.first_lock.acquire()
        self.second_lock.acquire()

    def first(self, printFirst: "Callable[[], None]") -> None:

        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self.first_lock.release()

    def second(self, printSecond: "Callable[[], None]") -> None:
        # printSecond() outputs "second". Do not change or remove this line.
        with self.first_lock:
            printSecond()
            self.second_lock.release()

    def third(self, printThird: "Callable[[], None]") -> None:

        # printThird() outputs "third". Do not change or remove this line.
        with self.second_lock:
            printThird()


class Solution:
    def distributeCandies(self, candies: int, num_people: int) -> List[int]:
        x = int(math.sqrt(candies * 2 + 0.25) - 0.5)
        result = [0] * num_people
        for i in range(num_people):
            m = x // num_people
            if (x % num_people) > i:
                m += 1

            result[i] = m * (i + 1) + m * (m - 1) // 2 * num_people
        result[x % num_people] += candies - x * (x + 1) // 2
        return result


class Input:
    candies: int
    num_people: int

    def __init__(self, candies: int, num_people: int):
        self.candies = candies
        self.num_people = num_people


def main():
    inputs = [Input(7, 4), Input(10, 3)]

    for input in inputs:
        result = Solution().distributeCandies(input.candies, input.num_people)
        print(result)


if __name__ == "__main__":
    main()
