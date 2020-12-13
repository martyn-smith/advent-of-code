from math import prod
from itertools import permutations

with open("13.txt") as f:
    arrival = int(f.readline())
    buses = [(i, int(j)) for i, j in enumerate(f.readline().split(",")) if j != "x"]
    ids, buses = [b[0] for b in buses], [b[1] for b in buses]

def get_wait_times(arrival):
    wait_times = []
    for bus in buses:
        i = 0
        while bus * i < arrival:
            i+=1
        wait_times.append((bus, (bus*i) - arrival))
    return wait_times

def get_timestamp():
    trial_time = 0
    while True:
        w = [w[1] for w in get_wait_times(trial_time)]
        if w == ids:
            return trial_time
        trial_time += 1

#part 1
w = min(get_wait_times(arrival), key= lambda x: x[1])
print(w[0] * w[1])

if any(b % c == 0 and b != c for b, c in permutations(buses, 2)):
    print("not coprime. Quitting...")
    exit(1)

#<923468884334109
B = prod(b for b in buses) # == 1748774091859561, higher than true answer
y = [B // b for b in buses]
z = [pow(i, -1, b) for i, b in zip(y, buses)]
print(B)
print(sum(i*j*k for i, j, k in zip(ids, y, z)) % B)
