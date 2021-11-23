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

def filter_red(items):
    if type(items) == dict:
        try:
            items.pop("red")
            for item in items.values():
                filter_red(item)
        except KeyError:
            pass
    elif type(items) == list:
        for item in items:
            filter_red(item)

def part_1():
    return find_nums(items)

def part_2():
    filter_red(items)
    return find_nums(items, True)

if __name__ == "__main__":
    print(part_1())
    print(part_2())
