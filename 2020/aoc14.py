import re

mask_srch = re.compile("^mask = (.*)")
mem_srch = re.compile("^mem\[(\d+)\] = (\d+)")

def mask_overwite(value, mask):
    """
    Returns the masked value for part 1.
    """
    #slightly cribbed, and I'm not super-happy with this implementation, since it constructs 
    #superfluous variables. That said, it's likely to be *much* faster than an if-then-else solution.
    #Still feel there's a nice number-theory solution to be found.
    return value & int(mask.replace("X", "1"), 2) | int(mask.replace("X", "0"), 2)

def part_1():
    memory = {}
    for line in lines:
        if m := mask_srch.match(line):
            mask = m.group(1)
        elif m := mem_srch.match(line):
            address, value = int(m.group(1)), int(m.group(2))
            value = mask_overwite(value, mask)
            memory[address] = value
    return memory

def get_addresses(address, mask):
    # If the bitmask bit is 0, the corresponding memory address bit is unchanged.
    # If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
    # If the bitmask bit is X, the corresponding memory address bit takes all values
    def address_overwrite(address):
        if address.find("X") == -1:
            return [int(address, 2)]
        else:
            return (address_overwrite(address.replace("X", "1", 1)) 
                   + address_overwrite(address.replace("X", "0", 1)))
    address = ''.join([m if m == "1" or m == "X" else a for a, m in zip(bin(address)[2:].zfill(36), mask)])
    return address_overwrite(address)

def part_2():
    memory = {}
    for line in lines:
        if m := mask_srch.match(line):
            mask = m.group(1)
        elif m := mem_srch.match(line):
            address, value = int(m.group(1)), int(m.group(2))
            addresses = get_addresses(address, mask)
            for addr in addresses:
                memory[addr] = value
    return memory

#setup
with open("14.txt") as f:
    lines = f.readlines()

#part 1
memory = part_1()
print(sum(memory[m] for m in memory))

#part 2
memory = part_2()
print(sum(memory[m] for m in memory))