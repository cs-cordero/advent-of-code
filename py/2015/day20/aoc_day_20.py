puzzle_input = 33100000

arr = [0] * (puzzle_input + 1)
end = puzzle_input // 10
x = 1
while x <= end:
    for index in range(x, end + 1, x):
        arr[index] += x * 10
        if arr[index] >= puzzle_input:
            end = index
            break
    x += 1

for i in range(len(arr)):
    if arr[i] >= puzzle_input:
        print(f"Solution 1: {i}")
        break

arr = [0] * (puzzle_input + 1)
end = puzzle_input // 11
x = 1
while x <= end:
    for index in range(x, end + 1, x):
        if index > x * 50:
            break
        arr[index] += x * 11
        if arr[index] >= puzzle_input:
            end = index
            break
    x += 1


for i in range(len(arr)):
    if arr[i] >= puzzle_input:
        print(f"Solution 2: {i}")
        break
