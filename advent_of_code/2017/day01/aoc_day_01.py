def solution(captcha):
    captcha_length = len(captcha)
    if captcha_length <= 1: return (0, 0)

    captcha_part_one = 0
    captcha_part_two = 0

    for i in range(captcha_length):
        captcha_part_one += (captcha[i]
                             if captcha[i] == captcha[(i+1)%captcha_length]
                             else 0)

        halfway_i = (i + captcha_length//2) % captcha_length
        captcha_part_two += (captcha[i]
                             if captcha[i] == captcha[halfway_i]
                             else 0)
    return captcha_part_one, captcha_part_two

if __name__ == '__main__':
    with open('aoc_day_01_input.txt') as f:
        s = [int(x) for x in f.read().strip()]
    print(solution(s))
