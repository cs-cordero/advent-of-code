def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def get_value(line):
    direction_str, value = line[0], int(line[1:])
    direction = 1 if direction_str == '+' else -1
    return direction * value


def solution():
    current_frequency = 0
    for line in read_file('input.txt'):
        current_frequency += get_value(line)
    return current_frequency


def solution2():
    memo = set()
    current_frequency = 0
    while True:
        for line in read_file('input.txt'):
            memo.add(current_frequency)
            current_frequency += get_value(line)
            if current_frequency in memo:
                return current_frequency

print(solution())
print(solution2())
