PUZZLE_INPUT = '580741'


def solution(n):
    recipes = ['3', '7']
    elf1 = 0
    elf2 = 1

    iterations = int(n)
    while len(recipes) < iterations + 10:
        score1, score2 = int(recipes[elf1]), int(recipes[elf2])
        new_recipes = list(str(score1 + score2))
        recipes.extend(new_recipes)
        elf1 = (elf1 + score1 + 1) % len(recipes)
        elf2 = (elf2 + score2 + 1) % len(recipes)

    return ''.join(recipes[iterations:iterations+10])


def solution2(n):
    recipes = ['3', '7']
    elf1 = 0
    elf2 = 1

    len_of_input = len(n)
    listified_n = list(n)
    while True:
        score1, score2 = int(recipes[elf1]), int(recipes[elf2])
        new_recipes = list(str(score1 + score2))
        recipes.extend(new_recipes)
        elf1 = (elf1 + score1 + 1) % len(recipes)
        elf2 = (elf2 + score2 + 1) % len(recipes)

        if recipes[-len_of_input:] == listified_n:
            return len(recipes) - len_of_input
        elif recipes[-len_of_input-1:-1] == listified_n:
            return len(recipes) - len_of_input - 1

    return len(recipes) - len_of_input


assert solution('9') == '5158916779', solution(9)
assert solution('5') == '0124515891'
assert solution('18') == '9251071085'
assert solution('2018') == '5941429882'
part1 = solution(PUZZLE_INPUT)
print(f'Part 1: {part1}')

assert solution2('51589') == 9
assert solution2('01245') == 5
assert solution2('92510') == 18
assert solution2('59414') == 2018
part2 = solution2(PUZZLE_INPUT)
print(f'Part 2: {part2}')
