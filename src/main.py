"""
main
"""

from typing import Optional

class MyCalendar:
    events: list[list[int]]
    def __init__(self):
        self.events = []

    def book(self, start: int, end: int) -> bool:
        events = self.events
        for event in events:
            l = max(event[0], start)
            r = min(event[1], end)
            if l < r:
                return False
        events.append([start, end])
        return True


class Input:
    book: list[list[int]]
    def __init__(self, book: list[list[int]]):
        self.book = book

def main():
    inputs: list[Input] = [
            Input([[10, 20], [15, 25], [20, 30]]),
    ]

    calendar = MyCalendar()
    for i in inputs:
        book = i.book
        for event in book:
            result = calendar.book(event[0], event[1])
            print(result)




if __name__ == "__main__":
    main()
