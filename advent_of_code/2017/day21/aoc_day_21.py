def solution(rules, iterations):
    pattern = ".#./..#/###"

    for _ in range(iterations):
        a = split_pattern(pattern)
        b = transform_squares(a, rules)
        pattern = group_squares(b)

    return pattern


def split_pattern(pattern):
    """
    1234
    5678  = 1234/5678 ==> [[12/56, 34/78]]
    """
    rows = pattern.split("/")
    size = 2 if len(rows) % 2 == 0 else 3

    result = []
    for height in range(0, len(rows), size):
        hrows = rows[height : height + size]
        split_rows = [
            [row[i : i + size] for i in range(0, len(row), size)] for row in hrows
        ]
        zipped_rows = ["/".join(z) for z in zip(*split_rows)]
        result.append(zipped_rows)
    return result


def transform_squares(squares, rules):
    for i in range(len(squares)):
        for j in range(len(squares[0])):
            inner_pattern = squares[i][j]
            squares[i][j] = rules[inner_pattern]
    return squares


def group_squares(squares):
    result = []
    for row in squares:
        split_squares = [square.split("/") for square in row]
        zipped_squares = "/".join(["".join(z) for z in zip(*split_squares)])
        result.append(zipped_squares)
    return "/".join(result)


def rotate_pattern(pattern):
    split_pattern = pattern.split("/")
    rotated_pattern = list(zip(*split_pattern[::-1]))
    return "/".join(["".join(row) for row in rotated_pattern])


def flip_pattern(pattern):
    split_pattern = pattern.split("/")
    return "/".join(split_pattern[::-1])


if __name__ == "__main__":
    rules = {}
    with open("aoc_day_21_input.txt", "r") as f:
        for line in f.readlines():
            rule, result = line.strip().split(" => ")
            for _ in range(4):
                rule = rotate_pattern(rule)
                rules[rule] = result
            rule = flip_pattern(rule)
            for _ in range(4):
                rule = rotate_pattern(rule)
                rules[rule] = result
    print(f'Part 1: {solution(rules, 5).count("#")}')
    print(f'Part 2: {solution(rules, 18).count("#")}')
