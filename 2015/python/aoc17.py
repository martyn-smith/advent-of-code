from itertools import combinations

total = 150

with open("../data/17.txt", "r") as f:
    data = list(int(l) for i, l in enumerate(f.readlines()))


def paths(target, bucket):
    paths = []
    for i in range(1, len(bucket)):
        for c in combinations(bucket, i):
            if sum(c) == target:
                paths += [c]
    return paths


def part_1():
    return len(paths(total, data))


def part_2():
    l = min(len(p) for p in paths(total, data))
    return len([p for p in paths(total, data) if len(p) == l])


if __name__ == "__main__":
    print(part_1())
    print(part_2())
