from collections import defaultdict


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def is_one_character_apart(first, second):
    differences = 0
    for i in range(len(first)):
        if first[i] != second[i]:
            differences += 1
        if differences > 1:
            return False
    return differences == 1


def solution():
    checksum_2s = 0
    checksum_3s = 0
    for line in read_file('input.txt'):
        memo = defaultdict(int)
        for letter in line:
            memo[letter] += 1
        if 2 in memo.values():
            checksum_2s += 1
        if 3 in memo.values():
            checksum_3s += 1
    return checksum_2s * checksum_3s


def solution2():  # brute force
    all_ids = [line for line in read_file('input.txt')]
    for i in range(len(all_ids)-1):
        for j in range(1, len(all_ids)):
            first, second = all_ids[i], all_ids[j]
            if is_one_character_apart(first, second):
                return ''.join(group[0] for group in zip(first, second)
                               if group[0] == group[1])


print(solution())
print(solution2())
