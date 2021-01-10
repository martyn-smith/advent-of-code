class Interpreter():
    #Note, we cast to int() so leading zeros are ALWAYS dropped.
    #additionally, the parameter to write to is ALWAYS positional, so is
    #ALWAYS zero, so is ALWAYS dropped.
    opcode_args = {
        1 : 3,
        2 : 3,
        3 : 1,
        4 : 1,
        99 : 0
    }

    def __init__(self):
        #this dict stores methods so has to be within init, apparently.
        self.opcodes = {
            1 : self.add,
            2 : self.multiply,
            3 : self.write,
            4 : self.read,
            99 : self.terminate #ERROR: cannot self terminate
        }
        with open("data/data/5.txt") as f:
            self.codes = [int(c) for c in f.read().strip("\n").split(",")]
        self.ctr = 0
        #self.test()

    def run(self):
        while True:
            """
            ABCDE
             1002

            DE - two-digit opcode,      02 == opcode 2
            C - mode of 1st parameter,  0 == position mode
            B - mode of 2nd parameter,  1 == immediate mode
            A - mode of 3rd parameter,  0 == position mode

            NOTE: leading zeros are OMITTED.  Even in the input file.
            
            NOTE: parameters than an instruction writes to will 
            ALWAYS be in position mode (0)

            Therefore, the shortest opcode is one char (3 or 4),
            longest is five.
            """
            code = self.codes[self.ctr]
            print(f"ctr = {self.ctr}, code = {code}")
            opcode = code % 10**2
            try:
                operation = self.opcodes[opcode]
            except KeyError:
                print(f"counter = {self.ctr}, code = {opcode}")
                exit()
            num_args = self.opcode_args[opcode]
            self.modes = ["immediate" if code // 10**i % 10 else "position" for i in range(num_args + 1, 1, -1)]
            try:
                assert(self.modes != [])
            except AssertionError:
                print(f"counter = {self.ctr}, code = {code}, args = {num_args}")
                exit()
            try:
                operation()
            except IndexError:
                print( f"counter = {self.ctr}, code = {code}, args = {num_args}")
            self.ctr += num_args + 1

    def add(self):
        args = [self.ctr + 1, self.ctr + 2, self.ctr + 3]
        args = [self.codes[arg] for arg in args]
        if self.modes[0] == "position":
            args[0] = self.codes[args[0]]
        if self.modes[1] == "position":
            args[1] = self.codes[args[1]]
        #print(f"writing {args[0]} + {args[1]} to pos {args[2]}")
        self.codes[args[2]] = args[0] + args[1]

    def multiply(self):
        args = [self.ctr + 1, self.ctr + 2, self.ctr + 3]
        args = [self.codes[arg] for arg in args]
        if self.modes[0] == "position":
            args[0] = self.codes[args[0]]
        if self.modes[1] == "position":
            args[1] = self.codes[args[1]]
        #print(f"writing {args[0]} + {args[1]} to pos {args[2]}")
        self.codes[args[2]] = args[0] * args[1]

    def read(self):
        a = self.ctr + 1 #MUST be position mode
        print(self.codes[self.codes[a]])

    def write(self):
        a = self.ctr + 1 #MUST be position mode
        self.codes[self.codes[a]] = int(input("enter a value to write: \n"))

    def terminate(self, _, __, ___):
        print(self.codes[0])
        exit()

if __name__ == "__main__":
    interpreter = Interpreter()
    interpreter.run()
