from collections import defaultdict, deque
from typing import Iterator, List, Optional, Tuple

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

        if value[-2:] != "\n":
            self.send(ord("\n"))

    def send(self, value: int) -> None:
        self.stdin.append(value)
        self.needs_input = False

    def has_input(self) -> bool:
        return bool(self.stdin)

    def has_output(self) -> bool:
        return bool(self.stdout)

    def read(self) -> Optional[int]:
        if self.stdout:
            return self.stdout.popleft()

    def readline(self) -> str:
        result = []
        while self.stdout:
            ch = chr(self.read())
            if ch == "\n":
                break
            result.append(ch)
        return "".join(result)

    def readlines(self) -> Iterator[str]:
        while self.stdout:
            yield self.readline()

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
