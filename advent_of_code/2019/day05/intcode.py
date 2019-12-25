from collections import defaultdict, deque
from typing import List, Optional, Tuple


class IntcodeComputer:
    def __init__(self, data: List[int]) -> None:
        self.registers = defaultdict(int)
        for i, value in enumerate(data):
            self.registers[i] = value

        self.current = 0
        self.halted = False
        self.needs_input = False
        self.stdin = deque()
        self.stdout = deque()

    def get_instruction(self) -> Tuple[int, int, int, int]:
        raw = self.registers[self.current]
        opcode = raw % 100
        raw //= 100
        mode1 = raw % 10
        raw //= 10
        mode2 = raw % 10
        raw //= 10
        mode3 = raw % 10
        return (
            opcode,
            (
                self._parse_value_with_mode(self.registers[self.current + 1], mode1),
                self._parse_value_with_mode(self.registers[self.current + 2], mode2),
                self._parse_value_with_mode(self.registers[self.current + 3], mode3),
            ),
        )

    def send(self, value: int) -> None:
        self.stdin.append(value)
        self.needs_input = False

    def read(self) -> Optional[int]:
        if self.stdout:
            return self.stdout.popleft()

    def _parse_value_with_mode(self, position: int, mode: int) -> int:
        return self.registers[position] if mode == 0 else position

    def run(self) -> None:
        if self.halted:
            return

        opcode, values = self.get_instruction()
        if opcode == 1:
            a, b, *_ = values
            target_position = self.registers[self.current + 3]
            self.registers[target_position] = a + b
            self.current += 4
        elif opcode == 2:
            a, b, *_ = values
            target_position = self.registers[self.current + 3]
            self.registers[target_position] = a * b
            self.current += 4
        elif opcode == 3:
            if not self.stdin:
                self.needs_input = True
                return
            target_position = self.registers[self.current + 1]
            self.registers[target_position] = self.stdin.popleft()
            self.current += 2
        elif opcode == 4:
            a, *_ = values
            self.stdout.append(a)
            self.current += 2
        elif opcode == 5:
            a, b, *_ = values
            self.current = b if a != 0 else self.current + 3
        elif opcode == 6:
            a, b, *_ = values
            self.current = b if a == 0 else self.current + 3
        elif opcode == 7:
            a, b, *_ = values
            target_position = self.registers[self.current + 3]
            self.registers[target_position] = 1 if a < b else 0
            self.current += 4
        elif opcode == 8:
            a, b, *_ = values
            target_position = self.registers[self.current + 3]
            self.registers[target_position] = 1 if a == b else 0
            self.current += 4
        elif opcode == 99:
            self.halted = True
        else:
            assert False, f"Invalid opcode: {opcode}"


def test_opcode_comparisons(data: List[int], value: int) -> int:
    computer = IntcodeComputer(data)
    computer.send(value)
    while not computer.halted:
        computer.run()
    return computer.read()


assert test_opcode_comparisons([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 2) == 0
assert test_opcode_comparisons([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8) == 1
assert test_opcode_comparisons([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 2) == 1
assert test_opcode_comparisons([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8) == 0
assert test_opcode_comparisons([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9) == 0
assert test_opcode_comparisons([3, 3, 1108, -1, 8, 3, 4, 3, 99], 2) == 0
assert test_opcode_comparisons([3, 3, 1108, -1, 8, 3, 4, 3, 99], 8) == 1
assert test_opcode_comparisons([3, 3, 1107, -1, 8, 3, 4, 3, 99], 2) == 1
assert test_opcode_comparisons([3, 3, 1107, -1, 8, 3, 4, 3, 99], 8) == 0
assert test_opcode_comparisons([3, 3, 1107, -1, 8, 3, 4, 3, 99], 9) == 0

jump_test1 = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
assert test_opcode_comparisons(jump_test1, 0) == 0
assert test_opcode_comparisons(jump_test1, 3) == 1

jump_test2 = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
assert test_opcode_comparisons(jump_test2, 0) == 0
assert test_opcode_comparisons(jump_test2, 3) == 1

big_test = [3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31]
big_test += [1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104]
big_test += [999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]
assert test_opcode_comparisons(big_test, 7) == 999
assert test_opcode_comparisons(big_test, 8) == 1000
assert test_opcode_comparisons(big_test, 9) == 1001
