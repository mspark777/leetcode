from __future__ import annotations
from typing import List
from collections import Counter


class Solution:
    def isNStraightHand(self, hand: List[int], group_size: int) -> bool:
        if len(hand) % group_size != 0:
            return False

        card_counts = Counter(hand)

        for card in hand:
            start_card = card
            while card_counts[start_card - 1]:
                start_card -= 1

            while start_card <= card:
                while card_counts[start_card]:
                    for next_card in range(start_card, start_card + group_size):
                        if not card_counts[next_card]:
                            return False
                        card_counts[next_card] -= 1

                start_card += 1

        return True


def main():
    input: list[tuple[list[int], int]] = [
        ([1, 2, 3, 6, 2, 3, 4, 7, 8], 3),
        ([1, 2, 3, 4, 5], 4),
    ]

    for hand, group_size in input:
        result = Solution().isNStraightHand(hand, group_size)
        print(result)


if __name__ == "__main__":
    main()
