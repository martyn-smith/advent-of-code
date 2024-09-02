from functools import lru_cache
total = 150

def paths(data, target):
    if len(data) == 0:
        return None
    elif data[0] == target:
        return [data[0]]
    elif data[0] > target:
        return paths(data[1:], target)
    else:
        pt = paths(data[1:], target - data[0])
        if pt is None:
            return pt
        return [data[0] + p for p in pt]

def part_1(data):
    return len(paths(data, total))

if __name__ == "__main__":
    with open("data/17.txt", "r") as f:
        data = [int(l) for i, l in enumerate(f.readlines())]
    print(part_1(data))
