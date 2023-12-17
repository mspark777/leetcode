from __future__ import annotations
from typing import List
from collections import defaultdict
from heapq import heappush, heappop


class Food:
    rating: int
    food: str

    def __init__(self, rating: int, food: str):
        self.rating = rating
        self.food = food

    def __lt__(self, other: Food):
        if self.rating == other.rating:
            return self.food < other.food
        return self.rating > other.rating


class FoodRatings:
    food_rating_map: dict[str, int]
    food_cuisine_map: dict[str, str]
    cuisine_food_map: dict[str, list[Food]]

    def __init__(self, foods: List[str], cuisines: List[str], ratings: List[int]):
        self.food_rating_map = {}
        self.food_cuisine_map = {}
        self.cuisine_food_map = defaultdict(list)

        for food, cuisine, rating in zip(foods, cuisines, ratings):
            self.food_rating_map[food] = rating
            self.food_cuisine_map[food] = cuisine
            heappush(self.cuisine_food_map[cuisine], Food(rating, food))

    def changeRating(self, food: str, newRating: int) -> None:
        self.food_rating_map[food] = newRating
        cuisineName = self.food_cuisine_map[food]
        heappush(self.cuisine_food_map[cuisineName], Food(newRating, food))

    def highestRated(self, cuisine: str) -> str:
        highest_rated = self.cuisine_food_map[cuisine][0]

        while self.food_rating_map[highest_rated.food] != highest_rated.rating:
            heappop(self.cuisine_food_map[cuisine])
            highest_rated = self.cuisine_food_map[cuisine][0]

        return highest_rated.food


def main():
    inputs = (
        (
            ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"],
            ["korean", "japanese", "japanese", "greek", "japanese", "korean"],
            [9, 12, 8, 15, 14, 7],
        ),
    )

    for foods, cuisines, ratings in inputs:
        ratings = FoodRatings(foods, cuisines, ratings)
        print(ratings.highestRated("korean"))
        print(ratings.highestRated("japanese"))
        ratings.changeRating("sushi", 16)
        print(ratings.highestRated("japanese"))
        ratings.changeRating("ramen", 16)
        print(ratings.highestRated("japanese"))


if __name__ == "__main__":
    main()
