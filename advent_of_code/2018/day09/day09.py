from collections import defaultdict, deque


def solution(players, last_marble):
    current_player = 0
    current_marble = 1
    scores = defaultdict(int)
    board = deque([0])
    while current_marble <= last_marble:
        if current_marble % 23 == 0:
            scores[current_player] += current_marble
            board.rotate(-7)
            scores[current_player] += board.popleft()
            board.rotate(1)
        else:
            board.rotate(1)
            board.appendleft(current_marble)

        current_marble += 1
        current_player = (current_player + 1) % players

    return max(scores.values())


assert solution(10, 1618) == 8317
assert solution(13, 7999) == 146373
assert solution(17, 1104) == 2764
assert solution(21, 6111) == 54718
assert solution(30, 5807) == 37305
print(solution(476, 71431))
print(solution(476, 71431 * 100))
