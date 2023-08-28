from __future__ import annotations


class MyStack:
    q0: list[int]
    q1: list[int]
    t: int

    def __init__(self):
        self.q0 = []
        self.q1 = []
        self.t = 0

    def push(self, x: int) -> None:
        self.q0.append(x)
        self.t = x

    def pop(self) -> int:
        while len(self.q0) > 1:
            self.t = self.q0.pop(0)
            self.q1.append(self.t)

        t = self.q0.pop(0)
        temp = self.q0
        self.q0 = self.q1
        self.q1 = temp
        return t

    def top(self) -> int:
        return self.t

    def empty(self) -> bool:
        return (len(self.q0) + len(self.q1)) < 1


def main():
    stack = MyStack()
    stack.push(1)
    stack.push(2)
    print(stack.top())
    stack.push(3)
    print(stack.top())


if __name__ == "__main__":
    main()
