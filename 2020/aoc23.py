"""
Advent of code day 23: playing find-the-lady with a crab.
"""
line = 463528179

def make_cups(line: int) -> list:
    return [int(c) for c in str(line)]

def unmake_cups(cups: list) -> int:
    idx = cups.index(1)
    return int(''.join([str(c) for c in cups[idx+1:] + cups[:idx]]))

def linkify(cup_list: list) -> dict:
    cups = {}
    for i in range(len(cup_list)-1):
        cups[cup_list[i]] = cup_list[i+1]
    cups[cup_list[-1]] = cup_list[0]
    return cups

def unlinkify(cup_dict: dict) -> list:
    start = cup_dict[1]
    cups = [start]
    c = cup_dict[start]
    while c != start:
        cups += [c]
        c = cup_dict[c]
    return cups

def move_linked(cups, curr_cup) -> int:
    """
    Implements a *crab* moving a circular array of cups, similar to part 1.
    Linked-list approach. Credit to u/fsed123 and u/Nastapoka - the basic idea is a dictionary,
    where the value is the next node. This spares the modular arithmetic, and any rearrangement,
    as you just have to detach/reattach rather than reassign.
    """
    def find_dest_cup(srch):
        srch -= 1
        while srch in (cup_1, cup_2, cup_3):
            srch -= 1
            if srch == 0:
                srch = l
        return (max(cups) if srch == 0 else srch)
    cup_1 = cups[curr_cup]
    cup_2 = cups[cup_1]
    cup_3 = cups[cup_2]
    cups[curr_cup] = cups[cup_3]
    dest_cup = find_dest_cup(curr_cup)
    tmp = cups[dest_cup]
    cups[dest_cup] = cup_1
    cups[cup_3] = tmp
    return cups[curr_cup]

def move(cups, curr_cup) -> int:
    """
    Implements a *crab* moving a circular array of cups according to the following rules:

    The crab picks up the three cups that are immediately clockwise of the current cup. 
    They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
    The crab selects a destination cup: the cup with a label equal to the current cup's label minus one. 
    If this would select one of the cups that was just picked up, the crab will keep subtracting one until it finds a cup that wasn't just picked up.
    If at any point in this process the value goes below the lowest value on any cup's label, 
    it wraps around to the highest value on any cup's label instead.
    The crab places the cups it just picked up so that they are immediately clockwise of the destination cup. 
    They keep the same order as when they were picked up.
    The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
    """
    def find_dest_cup(srch):
        srch -= 1
        while srch in (cup_1, cup_2, cup_3):
            srch -= 1
            if srch == 0:
                srch = l
        return (max(cups) if srch == 0 else srch)
    l = len(cups)
    idx = cups.index(curr_cup)
    cup_1, cup_2, cup_3 = (cups.pop(idx + 1 if idx + 1 < l else 0), 
                          cups.pop(idx + 1 if idx + 1 < l - 1 else 0), 
                          cups.pop(idx + 1 if idx + 1 < l - 2 else 0))
    dest_cup = find_dest_cup(curr_cup)
    dest_idx = cups.index(dest_cup)
    cups.insert(dest_idx + 1, cup_3)
    cups.insert(dest_idx + 1, cup_2)
    cups.insert(dest_idx + 1, cup_1)
    idx = cups.index(curr_cup)
    return cups[(idx + 1) % l]

def part_1():
    cups = make_cups(line)
    curr_cup = cups[0]
    for i in range(100):
        curr_cup = move(cups, curr_cup)
    return unmake_cups(cups)

def part_2():
    cups = make_cups(line)
    curr_cup = cups[0]
    cups = linkify(cups + [*range(max(cups), 1_000_001)])
    for i in range(10_000_001):
        curr_cup = move_linked(cups, curr_cup)
    idx = cups[1]
    return cups[1] * cups[cups[1]]

if __name__ == "__main__":
    print(part_1())
    print(part_2())