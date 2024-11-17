from functools import cache

gates = {}


def maybe_int(x):
    return int(x) if x.isdigit() else x


with open("../data/7.txt", "r") as f:
    for line in f.read().splitlines():
        lhs, gate_id = line.split(" -> ")
        if "NOT" in lhs:
            r = lhs.find("NOT ") + len("NOT ")
            gates[gate_id] = ("NOT", maybe_int(lhs[r:]))
        elif "AND" in lhs:
            s = lhs.find(" AND ")
            r = s + len(" AND ")
            gates[gate_id] = ("AND", (maybe_int(lhs[:s]), maybe_int(lhs[r:])))
        elif "OR" in lhs:
            s = lhs.find(" OR ")
            r = s + len(" OR ")
            gates[gate_id] = ("OR", (maybe_int(lhs[:s]), maybe_int(lhs[r:])))
        elif "LSHIFT" in lhs:
            s = lhs.find(" LSHIFT ")
            r = s + len(" LSHIFT ")
            gates[gate_id] = ("LSHIFT", (maybe_int(lhs[:s]), maybe_int(lhs[r:])))
        elif "RSHIFT" in lhs:
            s = lhs.find(" RSHIFT ")
            r = s + len(" RSHIFT ")
            gates[gate_id] = ("RSHIFT", (maybe_int(lhs[:s]), maybe_int(lhs[r:])))
        else:
            gates[gate_id] = maybe_int(lhs)


@cache
def solve(tgt):
    if type(tgt) is int:
        return tgt
    else:
        gate = gates[tgt]
        if type(gate) is int:
            return gate
        if gate[0] == "NOT":
            return ~solve(gate[1])
        elif gate[0] == "AND":
            return solve(gate[1][0]) & solve(gate[1][1])
        elif gate[0] == "OR":
            return solve(gate[1][0]) | solve(gate[1][1])
        elif gate[0] == "LSHIFT":
            return solve(gate[1][0]) << solve(gate[1][1])
        elif gate[0] == "RSHIFT":
            return solve(gate[1][0]) >> solve(gate[1][1])
        else:
            return solve(gate)


def part_1():
    return solve("a")


def part_2():
    x = solve("a")
    gates["b"] = x
    solve.cache_clear()
    return solve("a")


if __name__ == "__main__":
    print(part_1())
    print(part_2())
