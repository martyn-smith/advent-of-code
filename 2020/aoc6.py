#slight pain point here: since part 1 doesn't require knowing *who* selected a response,
#that information can be dropped. In part 2 it can't.
def parse_input():
    with open("6.txt") as f:
        entry = ""
        for line in f.readlines():
            if line.strip() == "":
                yield entry
                entry = ""
            else:
                entry += line.strip("\n")
        yield entry

def parse_input_2():
    with open("6.txt") as f:
        entry = []
        for line in f.readlines():
            if line.strip() == "":
                yield entry
                entry = []
            else:
                entry += [line.strip("\n")]
        yield entry

def count_positives(group):
    #set() already deduplicates
    return len(set(group))

def count_positives_2(group):
    if len(group) <= 1:
        return len(set(group[0]))
    responses = set(group[0])
    for e in group[1:]:
        if responses is None:
            return 0
        responses.intersection_update(set(e))
    return len(responses) if responses is not None else 0

print(sum(count_positives(entry) for entry in parse_input()))
print(sum(count_positives_2(entry) for entry in parse_input_2()))