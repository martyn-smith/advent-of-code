import re

with open("data/5.txt") as f:
    lines = f.readlines()


def nice_or_naughty(line):
    return (re.search(r"([aeiou].*){3}", line)
            and re.search(r"(\w)\1", line)
            and not re.search(r"(ab)|(cd)|(pq)|(xy)", line))

def nice_or_naughty_2(line):
    return (re.search(r"(\w{2}).*\1", line)
            and re.search(r"(\w)[^\1]\1", line))

def part_1():
    return sum(1 for line in lines if nice_or_naughty(line))


def part_2():
    return sum(1 for line in lines if nice_or_naughty_2(line))

if __name__ == "__main__":
    print(part_1())
    print(part_2())
