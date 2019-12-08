def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.rstrip()


class String:
    def __init__(self, string: str) -> None:
        self.string = string

    @property
    def memory_character_count(self) -> int:
        return len(self.string)

    @property
    def literal_character_count(self) -> int:
        count = 0
        i = 1
        while i < self.memory_character_count - 1:
            if self.string[i] == "\\":
                i += 1
                if i >= self.memory_character_count:
                    count += 1
                    break
                if self.string[i] in ("\\", '"'):
                    i += 1
                elif self.string[i] == "x":
                    i += 3
                else:
                    assert False, "Invariant"
                count += 1
                continue
            count += 1
            i += 1
        return count

    @property
    def expanded_character_count(self) -> int:
        count = 0
        for char in self.string:
            if char in ("\\", '"'):
                count += 1
            count += 1
        return count + 2


def solution():
    strings = [String(line) for line in read_file("aoc_day_08_input.txt")]
    part1 = sum(
        string.memory_character_count - string.literal_character_count
        for string in strings
    )
    part2 = sum(
        string.expanded_character_count - string.memory_character_count
        for string in strings
    )
    return part1, part2


assert String('""').memory_character_count == 2
assert String('""').literal_character_count == 0
assert String('""').expanded_character_count == 6
assert String('"abc"').memory_character_count == 5
assert String('"abc"').literal_character_count == 3
assert String('"abc"').expanded_character_count == 9
assert String('"aaa\\"aaa"').memory_character_count == 10
assert String('"aaa\\"aaa"').literal_character_count == 7
assert String('"aaa\\"aaa"').expanded_character_count == 16
assert String('"\\x27"').memory_character_count == 6
assert String('"\\x27"').literal_character_count == 1
assert String('"\\x27"').expanded_character_count == 11
print(solution())
