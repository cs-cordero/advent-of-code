
def solution(target):
    if target == 1:
        return 0
    result = find_ring(target, 1, 1)
    return result, sum(result)

def find_ring(target, num, width):
    min_in_ring = num + 1
    max_in_ring = num + ((width * 4) + 4)
    width += 2
    if min_in_ring <= target and target <= max_in_ring:
        return width // 2, find_height(target, min_in_ring, max_in_ring, width)
    return find_ring(target, max_in_ring, width)

def find_height(target, start_count, end_count, width):
    # check on right side of box
    top_right = start_count + width - 2
    top_left = top_right + width - 1
    bottom_left = top_left + width - 1
    bottom_right = bottom_left + width - 1

    if target <= top_right:
        offset_base = top_right
    elif target <= top_left:
        offset_base = top_left
    elif target <= bottom_left:
        offset_base = bottom_left
    else:
        offset_base = bottom_right

    return abs(offset_base - width // 2 - target)


print(solution(1))
print(solution(12))
print(solution(23))
print(solution(1024))
print(solution(361527))
