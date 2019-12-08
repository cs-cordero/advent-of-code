import re
import sys
from functools import reduce
from typing import Generator, List, NamedTuple, Optional

TEST_DATA = [
    "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
    "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
]


def read_file(filepath):
    if "test" not in sys.argv:
        with open(filepath) as f:
            for line in f.readlines():
                yield line.rstrip()
    else:
        for data in TEST_DATA:
            yield data


class ScoreTuple(NamedTuple):
    capacity: int
    durability: int
    flavor: int
    texture: int


class Ingredient(NamedTuple):
    capacity: int
    durability: int
    flavor: int
    texture: int
    calories: int

    def get_score(self, amount: int) -> ScoreTuple:
        return ScoreTuple(
            capacity=self.capacity * amount,
            durability=self.durability * amount,
            flavor=self.flavor * amount,
            texture=self.texture * amount,
        )

    def get_calories(self, amount: int) -> int:
        return self.calories * amount


def combination_generator(
    absolute_max_num: int,
    num_of_needed_categories: int,
    current: Optional[List[int]] = None,
) -> Generator[List[int], None, None]:
    current = current or []
    if len(current) == num_of_needed_categories - 1:
        yield current + [absolute_max_num - sum(current)]
        return

    max_num = absolute_max_num - sum(current)
    for i in range(max_num, -1, -1):
        next_combo = current + [i]
        yield from combination_generator(
            absolute_max_num, num_of_needed_categories, next_combo
        )


def solution(part2: bool = False) -> int:
    pattern = re.compile(
        r"^\w+:"
        r"[\s\w\,]*?"
        r"(?P<capacity>[-\d]+)"
        r"[\s\w\,]*?"
        r"(?P<durability>[-\d]+)"
        r"[\s\w\,]*?"
        r"(?P<flavor>[-\d]+)"
        r"[\s\w\,]*?"
        r"(?P<texture>[-\d]+)"
        r"[\s\w\,]*?"
        r"(?P<calories>[-\d]+)"
    )

    ingredients = []
    for line in read_file("aoc_day_15_input.txt"):
        regex_match = pattern.match(line)
        assert regex_match is not None
        _data = regex_match.groupdict()
        data = {key: int(value) for key, value in _data.items()}
        ingredients.append(Ingredient(**data))

    best_score = 0
    for amounts in combination_generator(100, len(ingredients)):
        if part2:
            calories = sum(
                ingredient.get_calories(amount)
                for ingredient, amount in zip(ingredients, amounts)
            )
            if calories != 500:
                continue

        score_tuples = [
            ingredient.get_score(amount)
            for ingredient, amount in zip(ingredients, amounts)
        ]
        cross_sums = [max(sum(cross), 0) for cross in zip(*score_tuples)]
        final_score = reduce(lambda x, y: x * y, cross_sums)
        if final_score > best_score:
            best_score = final_score

    return best_score


print(solution())
print(solution(part2=True))
