def solution(stream):
    score = 0
    last_score = 0
    garbage_score = 0

    flag_bang = False
    flag_garbage = False

    for char in stream:
        if flag_bang:
            # ignore the character if it's being escaped
            flag_bang = False
            continue

        if char == ">":
            # unflag the garbage flag if encounter unescaped >
            flag_garbage = False
        elif char == "!":
            # flag the ignore flag if encounter unescaped !
            flag_bang = True
        elif flag_garbage:
            # as long as the garbage flag is flagged, keep skipping
            garbage_score += 1
            continue
        elif char == "{":
            last_score += 1
        elif char == "}":
            score += last_score
            last_score -= 1
        elif char == "<":
            flag_garbage = True
    return score, garbage_score


if __name__ == "__main__":
    with open("aoc_day_09_input.txt") as f:
        s = [line.strip() for line in f.readlines()][0]
    answer = solution(s)
    print(f"Part 1: {answer[0]}")
    print(f"Part 2: {answer[1]}")
