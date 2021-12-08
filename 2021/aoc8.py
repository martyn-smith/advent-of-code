def get_display(l: str) -> tuple[list[str], list[str]]:
    samples, values = l.split("|")
    sample = [s for s in samples.split(" ")][:-1]
    value = [v for v in values.split(" ")][1:]
    return (sample, value)

def get_1478(sample: list[str]) -> int:
    return sum(1 for s in sample if len(s) == 2 or len(s) == 3 or len(s) == 4 or len(s) == 7)

with open("data/8.txt") as f:
    lines = [get_display(l.rstrip()) for l in f.readlines()]

def part_1():
    return sum(get_1478(l[1]) for l in lines)

if __name__ == "__main__":
    print(part_1())
