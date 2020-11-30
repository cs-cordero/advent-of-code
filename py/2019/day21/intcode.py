from collections import defaultdict, deque
from typing import List, Optional, Tuple

OPCOODE_TO_WRITE_PARAM_MAP = {
    1: 3,
    2: 3,
    3: 1,
    4: None,
    5: None,
    6: None,
    7: 3,
    8: 3,
    9: None,
    99: None,
}


class IntcodeComputer:
    def __init__(self, data: List[int]) -> None:
        self.registers = defaultdict(int)
        for i, value in enumerate(data):
            self.registers[i] = value

        self.reset()

    def reset(self) -> None:
        self.current = 0
        self.offset = 0
        self.halted = False
        self.needs_input = False
        self.stdin = deque()
        self.stdout = deque()

    def send_long(self, value: str) -> None:
        for letter in value:
            self.send(ord(letter))

    def send(self, value: int) -> None:
        self.stdin.append(value)
        self.needs_input = False

    def has_output(self) -> bool:
        return bool(self.stdout)

    def read(self) -> Optional[int]:
        if self.stdout:
            return self.stdout.popleft()

    def run_until_blocked(self) -> None:
        while self.halted is False and self.needs_input is False:
            self.run()

    def run(self) -> None:
        self._run()

    def _get_instruction(self) -> Tuple[int, int, int, int]:
        raw = self.registers[self.current]
        opcode = raw % 100
        write_param = OPCOODE_TO_WRITE_PARAM_MAP[opcode]

        raw //= 100
        mode1 = raw % 10
        raw //= 10
        mode2 = raw % 10
        raw //= 10
        mode3 = raw % 10
        return (
            opcode,
            (
                self._parse_value_with_mode(
                    self.registers[self.current + 1], mode1, write_param == 1
                ),
                self._parse_value_with_mode(
                    self.registers[self.current + 2], mode2, write_param == 2
                ),
                self._parse_value_with_mode(
                    self.registers[self.current + 3], mode3, write_param == 3
                ),
            ),
        )

    def _parse_value_with_mode(self, position: int, mode: int, write: bool) -> int:
        if write:
            return position + self.offset if mode == 2 else position
        elif mode == 1:
            return position
        return (
            self.registers[position]
            if mode == 0
            else self.registers[position + self.offset]
        )

    def _run(self) -> None:
        if self.halted:
            return

        opcode, values = self._get_instruction()
        if opcode == 1:
            a, b, c = values
            self.registers[c] = a + b
            self.current += 4
        elif opcode == 2:
            a, b, c = values
            self.registers[c] = a * b
            self.current += 4
        elif opcode == 3:
            if not self.stdin:
                self.needs_input = True
                return
            a, *_ = values
            self.registers[a] = self.stdin.popleft()
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
            a, b, c = values
            self.registers[c] = 1 if a < b else 0
            self.current += 4
        elif opcode == 8:
            a, b, c = values
            self.registers[c] = 1 if a == b else 0
            self.current += 4
        elif opcode == 9:
            a, *_ = values
            self.offset += a
            self.current += 2
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


# assert test_opcode_comparisons([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 2) == 0
# assert test_opcode_comparisons([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8) == 1
# assert test_opcode_comparisons([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 2) == 1
# assert test_opcode_comparisons([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8) == 0
# assert test_opcode_comparisons([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9) == 0
# assert test_opcode_comparisons([3, 3, 1108, -1, 8, 3, 4, 3, 99], 2) == 0
# assert test_opcode_comparisons([3, 3, 1108, -1, 8, 3, 4, 3, 99], 8) == 1
# assert test_opcode_comparisons([3, 3, 1107, -1, 8, 3, 4, 3, 99], 2) == 1
# assert test_opcode_comparisons([3, 3, 1107, -1, 8, 3, 4, 3, 99], 8) == 0
# assert test_opcode_comparisons([3, 3, 1107, -1, 8, 3, 4, 3, 99], 9) == 0
#
# jump_test1 = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
# assert test_opcode_comparisons(jump_test1, 0) == 0
# assert test_opcode_comparisons(jump_test1, 3) == 1
#
# jump_test2 = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
# assert test_opcode_comparisons(jump_test2, 0) == 0
# assert test_opcode_comparisons(jump_test2, 3) == 1
#
# big_test = [3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31]
# big_test += [1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104]
# big_test += [999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]
# assert test_opcode_comparisons(big_test, 7) == 999
# assert test_opcode_comparisons(big_test, 8) == 1000
# assert test_opcode_comparisons(big_test, 9) == 1001
#
test_set1 = [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
computer = IntcodeComputer(test_set1)
computer.run_until_blocked()
assert list(computer.stdout) == test_set1

# test_set2 = [3, 1985, 109, 2000, 204, -34]
# computer = IntcodeComputer(test_set2)
# computer.send(500)
# computer.run_until_blocked()
