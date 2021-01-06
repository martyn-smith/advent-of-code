"""
Advent of code day 8: building a basic interpreter for someone's not-a-Game-Boy(TM)
"""
from enum import Enum, auto

class RetCode(Enum):
    REPEATED = auto()
    HALTED = auto()

class Interpreter():

    def nop(self, arg):
        self.cursor += 1

    def acc(self, arg):
        self.accumulator += arg
        self.cursor += 1

    def jmp(self, arg):
        self.cursor += arg

    def __init__(self):
        self.opcodes = {
            "nop" : self.nop,
            "acc" : self.acc,
            "jmp" : self.jmp
        }
        self.cursor = 0
        self.accumulator = 0
        self.executed_lines = []
        with open("8.txt") as f:
            lines = [l for l in f.readlines()]
            self.ops = [l[:3] for l in lines]
            self.args = [int(l[4:]) for l in lines]

    def run(self) -> (RetCode, int):
        while(True):
            try:
                self.executed_lines.append(self.cursor)
                fetched = self.ops[self.cursor]
                arg = self.args[self.cursor]
                self.opcodes[fetched](arg)
                if self.cursor in self.executed_lines:
                    return RetCode.REPEATED, self.accumulator
            except IndexError:
                return RetCode.HALTED, self.accumulator

    def reset(self):
        self.cursor = 0
        self.accumulator = 0
        self.executed_lines = []

    def hunt(self) -> int:
        self.nop_lines = [i for i, op in enumerate(self.ops) if op == "nop"]
        self.jmp_lines = [i for i, op in enumerate(self.ops) if op == "jmp"]
        for i in self.nop_lines:
            self.ops[i] = "jmp"
            r = self.run()
            if r[0] == RetCode.HALTED:
                return self.accumulator
            self.reset()
            self.ops[i] = "nop" 
        for i in self.jmp_lines:
            self.ops[i] = "nop"
            r = self.run()
            if r[0]  == RetCode.HALTED:
                return self.accumulator
            self.reset()
            self.ops[i] = "jmp"

#setup
i = Interpreter()

#part 1
print(i.run()[1])

#part 2
print(i.hunt())