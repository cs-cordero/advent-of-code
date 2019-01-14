from collections import defaultdict
from dataclasses import dataclass
from typing import Tuple, List, Dict
import re
import sys


TEST_DATA = [
    'Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.',
    'Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.',
]


def read_file(filepath):
    if 'test' not in sys.argv:
        with open(filepath) as f:
            for line in f.readlines():
                yield line.rstrip()
    else:
        for data in TEST_DATA:
            yield data


@dataclass(repr=False, frozen=True)
class Reindeer:
    name: str
    speed: int
    max_active_time: int
    rest_time: int

    def __repr__(self):
        return self.name

    def distance_traveled_at_time(self, elapsed_time: int) -> int:
        active_rest_period = self.max_active_time + self.rest_time
        distance_in_one_active_period = self.speed * self.max_active_time
        active_periods = elapsed_time // active_rest_period
        remaining_time = min(elapsed_time % active_rest_period, self.max_active_time)

        distance_traveled = distance_in_one_active_period * active_periods
        distance_traveled += self.speed * remaining_time
        return distance_traveled


def solution() -> int:
    pattern = re.compile(r'^(?P<name>\w+)'
                         r' can fly '
                         r'(?P<speed>\d+)'
                         r'.*for '
                         r'(?P<max_active_time>\d+)'
                         r'.*rest for '
                         r'(?P<rest_time>\d+)'
                         r'.*$')

    reindeers: List[Reindeer] = []
    for line in read_file('aoc_day_14_input.txt'):
        regex_match = pattern.match(line)
        assert regex_match is not None
        data = regex_match.groupdict()
        name = data['name']
        speed = int(data['speed'])
        max_active_time = int(data['max_active_time'])
        rest_time = int(data['rest_time'])
        reindeers.append(Reindeer(name, speed, max_active_time, rest_time))

    def get_reindeers_in_the_lead(reindeers: List[Reindeer],
                                  elapsed_time: int) -> Tuple[Reindeer, int]:
        reindeer_by_distances_traveled = defaultdict(set)
        for reindeer in reindeers:
            distance_traveled = reindeer.distance_traveled_at_time(elapsed_time)
            reindeer_by_distances_traveled[distance_traveled].add(reindeer)

        return max(reindeer_by_distances_traveled.items(), key=lambda x: x[0])


    def get_scores_at_time(elapsed_time: int) -> Dict[Reindeer, int]:
        scores: Dict[Reindeer, int] = defaultdict(int)
        for i in range(1, elapsed_time+1):
            _, leading_reindeers = get_reindeers_in_the_lead(reindeers, i)
            for leading_reindeer in leading_reindeers:
                scores[leading_reindeer] += 1
        return scores

    scores = get_scores_at_time(2503)
    part1 = get_reindeers_in_the_lead(reindeers, 2503)
    part2 = max(scores.items(), key=lambda x: x[1])
    return part1, part2


print(solution())
