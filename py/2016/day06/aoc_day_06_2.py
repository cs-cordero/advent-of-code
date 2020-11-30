#!/usr/bin/python3


def find_smallest(some_dict):
    current_key = None
    current_min = None
    for key in some_dict:
        if current_min is None or some_dict[key] < current_min:
            current_key = key
            current_min = some_dict[key]
    return current_key


def main():
    letters = [{} for i in range(8)]
    with open("aoc_day_06_input.txt", "r") as f:
        for line in f:
            for i in range(len(line.strip())):
                letters[i][line[i]] = letters[i].get(line[i], 0) + 1

    final = []
    for i in range(8):
        final.append(find_smallest(letters[i]))
    return "".join(final)


if __name__ == "__main__":
    print(main())
