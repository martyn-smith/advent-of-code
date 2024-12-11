words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

with open("../data/1.txt", "r") as f:
    lines = f.readlines()

def simple_digits(line):
    digits = [int(c) for c in line if c.isdigit()]
    return 10 * digits[0] + digits[-1]

def get(i, line):
    if line[i].isdigit():
        return int(line[i])
    else:
        for (j, w) in enumerate(words):
            if line[i:].find(w) == 0:
                return j + 1
    return None

def word_digits(line):
    digits = [x for x in [get(i, line) for i in range(0,len(line))] if x is not None]
    return 10 * digits[0] + digits[-1]

def part_1():
    return sum(simple_digits(l) for l in lines)

def part_2():
    return sum(word_digits(l) for l in lines)

if __name__ == "__main__":
    print(part_1())
    print(part_2())

