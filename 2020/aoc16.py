import re
from math import prod

#all fields are two ranges
range_srch = re.compile("^(.*): (\d+)-(\d+) or (\d+)-(\d+)")

def parse_fields(fields):
    def parse_group(g):
        return (g[0], int(g[1]), int(g[2]), int(g[3]), int(g[4])) 
    return [parse_group(range_srch.match(line).groups()) for line in fields.split("\n")]

def parse_tickets(tickets):
    return [[int(i) for i in line.split(",")] for line in tickets.split("\n")[1:]]

def invalid_fields(ticket):
    return [t for t in ticket if not any(t in range(f[1], f[2]+1) or t in range(f[3], f[4]+1) for f in fields)]

def valid_fields(ticket):
    return [[f[0] for f in fields if t in range(f[1], f[2]+1) or t in range(f[3], f[4]+1)] for t in ticket]

def get_candidate_maps(valid_ticket_fields):
    return dict((f[0], [i for i in range(len(fields))
                                if all(f[0] in t[i] for t in valid_ticket_fields)])
                    for f in fields)

def solve(candidates):
    updated = True
    completed = []
    while updated:
        updated = False
        for f in candidates:
            if len(candidates[f]) == 1 and f not in completed:
                #print(f"eliminating... {f}")
                to_remove = candidates[f][0]
                for g in candidates:
                    if g != f and to_remove in candidates[g]:
                        candidates[g].remove(to_remove)
                        updated = True
                completed += f
    return candidates

def get_indices():
    valid_ticket_fields = [valid_fields(t) for t in nearby_tickets if invalid_fields(t) == []]
    possible_map = get_candidate_maps(valid_ticket_fields)
    mapping = solve(possible_map)
    return [mapping[f][0] for f in mapping if "departure" in f]

with open("16.txt") as f:
    tickets_file = f.read().split("\n\n")
    fields, my_ticket, nearby_tickets = (parse_fields(tickets_file[0]), 
                                           parse_tickets(tickets_file[1])[0], 
                                           parse_tickets(tickets_file[2]))

#part 1
print(sum(sum(invalid_fields(t)) for t in nearby_tickets))

#part 2
print(prod(my_ticket[i] for i in get_indices()))