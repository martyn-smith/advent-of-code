from itertools import combinations

def gravity_assist(noun = 12, verb = 2):

    def add(a, b, write_idx):
        #print(f"writing {a} + {b} to pos {write_idx}")
        codes[write_idx] = codes[a] + codes[b]

    def multiply(a, b, write_idx):
        #print(f"writing {a} * {b} to pos {write_idx}")
        codes[write_idx] = codes[a] * codes[b]

    def terminate(_, __, ___):
        pass

    opcodes = {
        1 : add,
        2 : multiply,
        99 : terminate
    }

    with open("./2.txt") as f:
        codes = [int(c) for c in f.read().strip("\n").split(",")]
        codes[1], codes[2] = noun, verb

    i = 0
    while True:
        op = opcodes[codes[i]]
        if op == terminate:
            break
        op(codes[i+1], codes[i+2], codes[i+3])
        i += 4

    return codes[0]

def hunt(target=19690720):
    for n, v in combinations(range(100), 2):
        if gravity_assist(n, v) == target:
            return n + v

if __name__ == "__main__":
    print(gravity_assist())
    print(100 * hunt())
