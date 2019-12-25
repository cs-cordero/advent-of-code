from collections import defaultdict
from typing import List


class IntcodeComputer:
    def __init__(self, data: List[int]) -> None:
        self.registers = defaultdict(int)
        for i, value in enumerate(data):
            self.registers[i] = value

        self.current = 0
        self.halted = False

    def get_active_opcode(self) -> int:
        return self.registers[self.current]

    def run(self) -> None:
        opcode = self.get_active_opcode()
        if opcode == 1:
            source_a, source_b, target_position = (
                self.registers[self.current + 1],
                self.registers[self.current + 2],
                self.registers[self.current + 3],
            )
            self.registers[target_position] = (
                self.registers[source_a] + self.registers[source_b]
            )
            self.current += 4
        elif opcode == 2:
            source_a, source_b, target_position = (
                self.registers[self.current + 1],
                self.registers[self.current + 2],
                self.registers[self.current + 3],
            )
            self.registers[target_position] = (
                self.registers[source_a] * self.registers[source_b]
            )
            self.current += 4
        elif opcode == 99:
            self.halted = True
        else:
            assert False, "Invalid opcode"
