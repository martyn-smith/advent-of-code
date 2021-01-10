def get_total_mass():
    def get_fuel(module_mass):
        return (module_mass//3 - 2)
    with open("data/data/1.txt") as f:
        total_fuel = sum(get_fuel(int(line)) for line in f)
    return total_fuel

def get_total_mass_2():
    def marginal_fuel(mass):
        return mass // 3 -2
    def total_fuel(module_mass):
        fuel = marginal_fuel(module_mass)
        m = marginal_fuel(fuel)
        while m > 0:
            fuel += m
            m = marginal_fuel(m)
        return fuel
    with open("data/data/1.txt") as f:
        total_fuel = sum(total_fuel(int(line)) for line in f)
    return total_fuel

if __name__ == "__main__":
    print(get_total_mass())
    print(get_total_mass_2())