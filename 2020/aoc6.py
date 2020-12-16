"""
Advent of code 6: we're fudging some declarations forms.
slight pain point here: since part 1 doesn't require knowing *who* selected a response,
that information can be dropped. In part 2 it can't.
"""
with open("6.txt") as f:
    lines = f.readlines()

def parse_input():
    entry = ""
    for line in lines:
        if line.strip() == "":
            yield entry
            entry = ""
        else:
            entry += line.strip("\n")
    yield entry

def parse_input_2():
    entry = []
    for line in lines:
        if line.strip() == "":
            yield entry
            entry = []
        else:
            entry += [line.strip("\n")]
    yield entry

def count_positives(group: str) -> int:
    """
    Self-explanatory. Uses set(), which deduplicates for us.
    """
    return len(set(group))

def count_positives_2(group: list) -> int:
    """
    Groupwise response counting - for which all parties answered yes.
    """
    if len(group) <= 1:
        return len(set(group[0]))
    responses = set(group[0])
    for g in group[1:]:
        if responses is None:
            return 0
        responses.intersection_update(set(g))
    return len(responses) if responses is not None else 0

#part 1
print(sum(count_positives(entry) for entry in parse_input()))

#part 2
print(sum(count_positives_2(entry) for entry in parse_input_2()))