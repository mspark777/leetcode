class MyHashSet:
    nums: list[bool]

    def __init__(self):
        self.nums = [False for i in range(1000001)]

    def add(self, key: int) -> None:
        self.nums[key] = True

    def remove(self, key: int) -> None:
        self.nums[key] = False

    def contains(self, key: int) -> bool:
        return self.nums[key]


def main():
    my_hash_set = MyHashSet()
    my_hash_set.add(1)
    my_hash_set.add(2)
    print(my_hash_set.contains(1))
    print(my_hash_set.contains(3))
    my_hash_set.add(2)
    print(my_hash_set.contains(2))
    my_hash_set.remove(2)
    print(my_hash_set.contains(2))


if __name__ == "__main__":
    main()
