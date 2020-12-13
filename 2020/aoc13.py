from math import prod

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
        #print(w, ids)
        #print(trial_time)
        if w == ids:
            return trial_time
        trial_time += 1


w = min(get_wait_times(arrival), key= lambda x: x[1])
print(w[0] * w[1])

#<1748774091859561
for b in buses:
    for c in buses:
        if b % c == 0 and b != c:
            print(f"{b} and {c} are not coprime")
print(prod(b for b in buses))
