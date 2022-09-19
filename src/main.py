"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def findDuplicate(self, paths: list[str]) -> list[list[str]]:
        path_map: dict[str, list[str]] = {}
        for path in paths:
            segments = path.split(" ")
            root = segments[0]
            for i in range(1, len(segments)):
                file = segments[i]
                sep = file.find("(")
                name = file[0:sep]
                content = file[sep:-1]
                filepath = f"{root}/{name}"
                if content not in path_map:
                    path_map[content] = [filepath]
                else:
                    path_map[content].append(filepath)

        return [value for value in path_map.values() if len(value) > 1]


class Input:
    paths: list[str]

    def __init__(self, paths: list[str]):
        self.paths = paths


def main():
    inputs: list[Input] = [
        Input(
            [
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
                "root 4.txt(efgh)",
            ]
        ),
        Input(
            [
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
            ]
        ),
        Input(
            [
                "root/a 1.txt(abcd) 2.txt(efsfgh)",
                "root/c 3.txt(abdfcd)",
                "root/c/d 4.txt(efggdfh)",
            ]
        ),
    ]

    solution = Solution()
    for input in inputs:
        paths = input.paths
        result = solution.findDuplicate(paths)
        print(result)


if __name__ == "__main__":
    main()
