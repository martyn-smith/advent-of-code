"""
Advent of Code day 1: The Rocket Equation
"""


def get_fuel(module_mass):
    return module_mass // 3 - 2


def get_recursive_fuel(module_mass):
    fuel = get_fuel(module_mass)
    m = get_fuel(fuel)
    while m > 0:
        fuel += m
        m = get_fuel(m)
    return fuel


with open("data/1.txt") as f:
    lines = f.readlines()


def part_1():
    return sum(get_fuel(int(line)) for line in lines)


def part_2():
    return sum(get_recursive_fuel(int(line)) for line in lines)


if __name__ == "__main__":
    print(part_1())
    print(part_2())
