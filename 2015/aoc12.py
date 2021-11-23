import json

with open("data/12.txt") as f:
    items = [json.loads(line) for line in f.readlines()]

def find_nums(items, no_red = False):
    if type(items) == int:
        return items
    elif type(items) == list:
        return sum(find_nums(i, no_red) for i in items)
    elif type(items) == dict:
        if no_red and "red" in items.values():
            return 0
        else:
            return sum(find_nums(i, no_red) for i in list(items.values()))
    else:
        return 0

def part_1():
    return find_nums(items)

def part_2():
    return find_nums(items, True)

if __name__ == "__main__":
    print(part_1())
    print(part_2())
