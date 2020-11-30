#!/usr/bin/python3


def find_largest(some_dict):
    current_key = None
    current_max = 0
    for key in some_dict:
        if some_dict[key] > current_max:
            current_key = key
            current_max = some_dict[key]
    return current_key


def main():
    letters = [{} for i in range(8)]
    with open("aoc_day_06_input.txt", "r") as f:
        for line in f:
            for i in range(len(line.strip())):
                letters[i][line[i]] = letters[i].get(line[i], 0) + 1

    final = []
    for i in range(8):
        final.append(find_largest(letters[i]))
    return "".join(final)


if __name__ == "__main__":
    print(main())
