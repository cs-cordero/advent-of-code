import re
import sys
from dataclasses import dataclass, field
from itertools import permutations
from typing import Dict, Tuple

TEST_DATA = [
    "Alice would gain 54 happiness units by sitting next to Bob.",
    "Alice would lose 79 happiness units by sitting next to Carol.",
    "Alice would lose 2 happiness units by sitting next to David.",
    "Bob would gain 83 happiness units by sitting next to Alice.",
    "Bob would lose 7 happiness units by sitting next to Carol.",
    "Bob would lose 63 happiness units by sitting next to David.",
    "Carol would lose 62 happiness units by sitting next to Alice.",
    "Carol would gain 60 happiness units by sitting next to Bob.",
    "Carol would gain 55 happiness units by sitting next to David.",
    "David would gain 46 happiness units by sitting next to Alice.",
    "David would lose 7 happiness units by sitting next to Bob.",
    "David would gain 41 happiness units by sitting next to Carol.",
]


def read_file(filepath):
    if "test" not in sys.argv:
        with open(filepath) as f:
            for line in f.readlines():
                yield line.rstrip()
    else:
        for data in TEST_DATA:
            yield data


@dataclass(repr=False)
class Person:
    name: str
    preferences: Dict["Person", int] = field(default_factory=dict)

    def __hash__(self):
        return hash((self.name,))

    def __repr__(self):
        return self.name

    def calculate_happiness(self, seating_arrangement: Tuple["Person", ...]) -> int:
        my_seat = seating_arrangement.index(self)
        to_left = seating_arrangement[(my_seat - 1) % len(seating_arrangement)]
        to_right = seating_arrangement[(my_seat + 1) % len(seating_arrangement)]
        return self.preferences[to_left] + self.preferences[to_right]


def solution(part2: bool = False) -> int:
    pattern = re.compile(
        r"^(?P<source>\w+)"
        r".*"
        r"(?P<affect>lose \d+|gain \d+)"
        r".*next to "
        r"(?P<target>\w+)\.$"
    )

    people: Dict[str, Person] = {}
    for line in read_file("aoc_day_13_input.txt"):
        regex_match = pattern.match(line)
        assert regex_match is not None
        data = regex_match.groupdict()
        direction, _affect = data["affect"].split(" ")

        source = data["source"]
        affect = int(_affect) * (-1 if direction == "lose" else 1)
        target = data["target"]

        source_person = people.setdefault(source, Person(source))
        target_person = people.setdefault(target, Person(target))
        source_person.preferences[target_person] = affect

    if part2:
        me = Person("Me")
        for name, person in people.items():
            person.preferences[me] = 0
            me.preferences[person] = 0
        people[me.name] = me

    max_happiness = (1 << 31) * -1
    for seating_arrangement in permutations(people.values()):
        happiness = sum(
            map(
                lambda seat: seat.calculate_happiness(seating_arrangement),
                seating_arrangement,
            )
        )
        if happiness > max_happiness:
            max_happiness = happiness
    return max_happiness


print(solution())
print(solution(part2=True))
