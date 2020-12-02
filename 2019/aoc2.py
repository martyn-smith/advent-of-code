def gravity_assist():

    def add(a, b, write_idx):
        #print(f"writing {a} + {b} to pos {write_idx}")
        codes[write_idx] = codes[a] + codes[b]

    def multiply(a, b, write_idx):
        #print(f"writing {a} * {b} to pos {write_idx}")
        codes[write_idx] = codes[a] * codes[b]

    def terminate(_, __, ___):
        print(codes[0])
        exit()

    opcodes = {
        1 : add,
        2 : multiply,
        99 : terminate
    }

    with open("./2.txt") as f:
        codes = [int(c) for c in f.read().strip("\n").split(",")]
        codes[1], codes[2] = 12, 2

    i = 0
    while True:
        opcodes[codes[i]](codes[i+1], codes[i+2], codes[i+3])
        i += 4

if __name__ == "__main__":
    gravity_assist()
