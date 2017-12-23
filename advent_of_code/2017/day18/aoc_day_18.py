from multiprocessing import Event, Pipe, Process, Queue, Value
from collections import defaultdict
from functools import wraps

class Registers:
    def __init__(self, program_id, queue_out, queue_in, event, instructions,
            counter=None):
        self.registers = defaultdict(int)
        self.registers['p'] = int(program_id)
        self.queue_out = queue_out
        self.queue_in = queue_in
        self.event = event
        self.instructions = instructions
        self.instruction_pointer = 0
        self.counter = counter

    def _sanitize_inputs(fn):
        @wraps(fn)
        def wrapper(self, *args):
            def sanitize(val):
                return val if val.isalpha() else int(val)

            if len(args) == 1:
                return fn(self, sanitize(args[0]))
            else:
                x = sanitize(args[0])
                y = sanitize(args[1])
                y = self.registers.get(y, y)
                return fn(self, x, y)
        return wrapper

    def is_set(self):
        return self.event.is_set()

    @_sanitize_inputs
    def send(self, x):
        if self.counter:
            self.counter.value += 1
        self.queue_out.put(self.registers[x])

    @_sanitize_inputs
    def set_register(self, x, y):
        self.registers[x] = y

    @_sanitize_inputs
    def add_to_register(self, x, y):
        self.registers[x] += y

    @_sanitize_inputs
    def multiply_register(self, x, y):
        self.registers[x] *= y

    @_sanitize_inputs
    def mod_register(self, x, y):
        self.registers[x] %= y

    def recover(self, x):
        self.event.set()
        while self.queue_in.empty():
            pass
        self.registers[x] = self.queue_in.get()
        self.event.clear()

    @_sanitize_inputs
    def jump_size(self, x, y):
        return y if self.registers.get(x, x) > 0 else 1

    @property
    def action_map(self):
        return {
            'snd': self.send,
            'set': self.set_register,
            'add': self.add_to_register,
            'mul': self.multiply_register,
            'mod': self.mod_register,
            'rcv': self.recover,
            'jgz': self.jump_size
        }

    def start(self):
        while True:
            action, params = self.instructions[self.instruction_pointer]
            step = self.action_map[action](*params)
            if step and isinstance(step, str):
                print(action, params, step, self.registers.get('p'))
            try:
                self.instruction_pointer += step or 1
            except:
                print(action, type(self.instruction_pointer), type(step))
                raise

def solution(instructions):
    def run(register):
        register.start()
    a, b = Queue(), Queue()
    counter = Value('i')
    register_0 = Registers(0, a, b, Event(), instructions)
    register_1 = Registers(1, b, a, Event(), instructions, counter=counter)
    proc_1 = Process(target=run, args=(register_0,))
    proc_1.daemon = True
    proc_1.start()

    proc2 = Process(target=run, args=(register_1,))
    proc2.daemon = True
    proc2.start()
    while (not (register_0.is_set() and register_1.is_set())
            or (a.qsize() or b.qsize())):
        pass
    return counter.value


if __name__ == '__main__':
    with open('aoc_day_18_input.txt', 'r') as f:
        instructions = [(line.split()[0], line.split()[1:])
                        for line in f.readlines()]
    print(f'Part 2: {solution(instructions)}')
