with open("data/8.txt", "r") as f:
    raws = f.readlines()
    literals = [eval(r) for r in raws]

def part_1():
    return sum(len(r[:-1]) - len(l) for l, r in zip(literals, raws))

if __name__ == "__main__":
    print(part_1())
