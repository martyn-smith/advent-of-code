with open("10.txt") as f:
    lines = f.readlines()

jolts = [int(l) for l in lines]
jolts.sort()
jolts.insert(0, 0)
jolts.append(max(jolts) + 3)

deltas = [i - j for i, j in zip(jolts[1:], jolts[:-1])]
print(deltas.count(1) * deltas.count(3))

paths = {}
def find_valid_paths(jolts):
    if jolts[0] in paths: #already calculated
        return 1
    if len(jolts) == 1: #endpoint
        paths[jolts[-1]] = 0
        return 1
    path_count = sum(1 for i in range(1,4) if jolts[0]+i in paths)
    if path_count:
        paths[jolts[0]] = path_count
        return 1
    else: #no precalculation yet
        for i in range(len(jolts) - 1):
            path_count = 0
            if jolts[i+1] - jolts[0] <= 3:
                find_valid_paths(jolts[i+1:])
                paths[jolts[i+1]] = 1
            return 1

find_valid_paths(jolts)
print(paths)