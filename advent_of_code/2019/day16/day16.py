from typing import List


def solution1(data: List[int]) -> str:
    phases = 100
    signal = data

    for phase in range(phases):
        next_signal = []
        for i in range(len(signal)):
            pattern = get_pattern(i + 1)
            value = 0
            for i, val in enumerate(signal):
                p_val = pattern[i % len(pattern)]
                value += val * p_val
            value = abs(value) % 10
            next_signal.append(abs(value) % 10)
        signal = next_signal
    return "".join(map(str, signal[:8]))


def solution2(data: List[int]) -> str:
    signal = data.copy() * 10000
    offset = int("".join(map(str, signal[:7])))
    for phase in range(100):
        perform_phase(signal, offset)
    return "".join(map(str, signal[offset : offset + 8]))


def get_pattern(k: int) -> List[int]:
    pattern = [value for value in (0, 1, 0, -1) for _ in range(k)]
    return pattern[1:] + [0]


def perform_phase(signal: List[int], offset: int) -> None:
    for i in range(len(signal) - 2, offset - 1, -1):
        signal[i] = (signal[i] + signal[i + 1]) % 10


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().strip()))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
