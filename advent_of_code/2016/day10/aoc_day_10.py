#!/bin/python3

import re


class Bot(object):
    def __init__(self, bot_id, bot_type):
        self.id = bot_id
        self.type = bot_type
        self.data = []
        self.low = None
        self.high = None


def main():
    bot_queue = []
    bots = {}  # dict of bots, keyed by id
    outputs = {}  # dict of outputs, keyed by id
    with open('aoc_day_10_input.txt', 'r') as f:
        for line in f:
            m = re.match(r'^bot\s*(\d+)\D*(bot|output)\D*(\d+)\D*(bot|output)\D*(\d+)', line)
            n = re.match(r'^value\s*(\d+)\D*(\d+)', line)
            if m:
                bot_id, low_location, low, high_location, high = m.groups()
                bot = bots.setdefault(bot_id, Bot(bot_id, 'bot'))
                bot.id = bot_id
                if low_location.upper() == 'BOT':
                    bot.low = bots.setdefault(low, Bot(low, 'bot'))
                else:
                    bot.low = outputs.setdefault(low, Bot(low, 'output'))

                if high_location.upper() == 'BOT':
                    bot.high = bots.setdefault(high, Bot(high, 'bot'))
                else:
                    bot.high = outputs.setdefault(high, Bot(high, 'output'))
            elif n:
                data, bot_id = n.groups()
                bot = bots.setdefault(bot_id, Bot(bot_id, 'bot'))
                bot.data.append(int(data))
                if len(bot.data) > 1:
                    bot_queue.append(bot)

    while bot_queue:
        bot = bot_queue.pop(0)
        if len(bot.data) < 2: continue
        while len(bot.data) == 2:
            bot.data.sort()
            lower, higher = bot.data.pop(0), bot.data.pop()
            if (lower, higher) == (17, 61):
                print('Part 1: %s' % bot.id)
            bot.low.data.append(lower)
            bot.high.data.append(higher)
        if bot.low.type == 'bot' and len(bot.low.data) > 1:
            bot_queue.append(bot.low)
        if bot.high.type == 'bot' and len(bot.high.data) > 1:
            bot_queue.append(bot.high)

    part_2 = outputs['0'].data[0] * outputs['1'].data[0] * outputs['2'].data[0]
    print('Part 2: %d' % part_2)


if __name__ == '__main__':
    main()
