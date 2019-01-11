from typing import Generator, Tuple


def break_string(string: str) -> Generator[Tuple[int, int], None, None]:
    i = 0
    while i < len(string):
        count = 0
        current_digit = string[i]
        while i < len(string) and string[i] == current_digit:
            count += 1
            i += 1
        yield (str(count), str(current_digit))


def solution(value: str, iterations: int) -> str:
    for i in range(iterations):
        value = ''.join(''.join(tup) for tup in break_string(value))
    return value

assert solution('1', 5) == '312211'
PUZZLE_INPUT = '1113222113'
part1 = len(solution(PUZZLE_INPUT, 40))
print(f'Part 1: {part1}')
part2 = len(solution(PUZZLE_INPUT, 50))
print(f'Part 2: {part2}')
