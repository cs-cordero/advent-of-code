from typing import Union


class LetterNumber:
    def __init__(self, value: Union[str, int]) -> None:
        self.underlying_int = (
            value if isinstance(value, int)
            else LetterNumber.convert_letternumber_to_number(value)
        )

    def __add__(self, other: Union[int, 'LetterNumber']) -> 'LetterNumber':
        value = other if isinstance(other, int) else other.underlying_int
        return LetterNumber(self.underlying_int + value)

    @staticmethod
    def convert_letternumber_to_number(string: str) -> int:
        place = len(string)
        ord_a = ord('a')
        value = 0
        for character in string:
            value += (ord(character) - ord_a + 1) * (26 ** (place - 1))
            place -= 1
        return value

    def __repr__(self) -> str:
        ord_a = ord('a')
        string = ''

        remaining = self.underlying_int
        while remaining > 0:
            remaining -= 1
            offset = remaining % 26
            string = chr(ord_a + offset) + string
            remaining -= offset
            remaining //= 26
        return string

    @property
    def has_increasing_straight(self) -> bool:
        representation = self.__repr__()
        for i in range(2, len(representation)):
            a = ord(representation[i-2])
            b = ord(representation[i-1])
            c = ord(representation[i-0])
            if c - b == 1 and b - a == 1:
                return True
        return False

    @property
    def has_no_prohibited_letters(self) -> bool:
        representation = self.__repr__()
        return not any(letter in representation for letter in ('i', 'o', 'l'))

    @property
    def has_two_pairs(self) -> bool:
        representation = self.__repr__()
        pairs = set()
        for i in range(1, len(representation)):
            a = representation[i-1]
            b = representation[i-0]
            if a == b:
                pairs.add(f'{a}{b}')
        return len(pairs) >= 2

    def get_next(self) -> 'LetterNumber':
        next_number = self + 1
        while True:
            a = next_number.has_increasing_straight
            b = next_number.has_no_prohibited_letters
            c = next_number.has_two_pairs
            if a and b and c:
                return next_number

            def get_index_safely(representation: str, value: str) -> int:
                try:
                    return representation.index(value)
                except ValueError:
                    return -1

            if not b:
                while True:
                    representation = next_number.__repr__()
                    indexes_of_prohibited_letters = [get_index_safely(representation, value)
                                                     for value in ('i', 'o', 'l')]

                    if all(value == -1 for value in indexes_of_prohibited_letters):
                        break

                    index_of_prohibited_letter = min(
                        value for value in indexes_of_prohibited_letters
                        if value >= 0
                    )

                    character = representation[index_of_prohibited_letter]
                    next_character = chr((ord(character)+1-97) % 26 + 97)

                    next_number = LetterNumber(
                        representation[:index_of_prohibited_letter]
                        + next_character
                        + (len(representation[index_of_prohibited_letter+1:]) * 'a')
                    )
            else:
                next_number += 1


PUZZLE_INPUT = 'hepxcrrq'
part1 = LetterNumber(PUZZLE_INPUT).get_next()
part2 = part1.get_next()
print(part1, part2)
