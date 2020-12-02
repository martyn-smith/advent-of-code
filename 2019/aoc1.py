def get_total_mass():
    def get_fuel(module_mass):
        return (module_mass//3 - 2)
    with open("./1.txt") as f:
        total_fuel = sum(get_fuel(int(line)) for line in f)
    return total_fuel

if __name__ == "__main__":
    print(get_total_mass())