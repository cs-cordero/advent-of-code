from itertools import permutations


def solution(lines):
    valid = 0
    for line in lines:
        line = line.strip().split(" ")
        seen = set()
        for word in line:
            if word in seen:
                break
            for perm in ("".join(s) for s in permutations(word)):
                seen.add(perm)
        else:
            valid += 1
    return valid


if __name__ == "__main__":
    with open("aoc_day_04_input.txt") as f:
        s = f.readlines()
    print(solution(s))
