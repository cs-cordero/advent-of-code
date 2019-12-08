def solution(instructions, repeat_sighting=False):
    num_banks = len(instructions)
    memory = set()
    configuration = "".join(map(str, instructions))
    steps = 0
    while configuration not in memory:
        if not (memory and repeat_sighting):
            memory.add(configuration)
        max_i = find_max(instructions)
        base_distribution_amount = instructions[max_i] // num_banks
        overflow_amount = instructions[max_i] % num_banks
        instructions[max_i] = base_distribution_amount
        i = (max_i + 1) % num_banks
        while i != max_i:
            instructions[i] += base_distribution_amount
            if overflow_amount:
                instructions[i] += 1
                overflow_amount -= 1
            i = (i + 1) % num_banks
        configuration = "".join(map(str, instructions))
        steps += 1
    return steps


def find_max(arr):
    current_max = 0
    for i in range(1, len(arr)):
        if arr[i] > arr[current_max]:
            current_max = i
    return current_max


if __name__ == "__main__":
    with open("aoc_day_06_input.txt") as f:
        s = list(map(int, f.readlines()[0].strip().split("\t")))
    print(solution([0, 2, 7, 0]))
    print(solution(s))
    print(solution(s, repeat_sighting=True))
