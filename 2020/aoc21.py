"""
Advent of code day 21: set theory.
"""

with open("data/21.txt") as f:
    lines = f.readlines()

all_allergens = {}
all_ingredients = set()

# generates all allergens and all ingredients, using elimination for the former.
# we could abstract ingredient/allergen split into a function, since it's used later. But could also not.
for line in lines:
    line = line.split("(")
    line_ingredients = line[0].strip().split(" ")
    line_allergens = line[1].strip("\n")[9:-1].replace(" ", "").split(",")
    all_ingredients.update(set(line_ingredients))
    for allergen in line_allergens:
        if allergen not in all_allergens:
            all_allergens[allergen] = set(line_ingredients)
        else:
            all_allergens[allergen] = all_allergens[allergen].intersection(
                set(line_ingredients)
            )

# gaussian elimination of allergens
while sum(1 for a in all_allergens if len(all_allergens[a]) > 1) > 1:
    for allergen in all_allergens:
        if len(all_allergens[allergen]) == 1:
            for other in all_allergens:
                if other != allergen:
                    all_allergens[other] = all_allergens[other].difference(
                        all_allergens[allergen]
                    )

# generates a total (flat) list of allergens and finds the ingredients not included.
all_allergen_ingredients = set()
for a in all_allergens:
    all_allergen_ingredients.update(all_allergens[a])
non_allergen_ingredients = all_ingredients.difference(all_allergen_ingredients)


def part_1():
    return sum(
        sum(
            1
            for ingredient in line.split("(")[0].strip().split(" ")
            if ingredient in non_allergen_ingredients
        )
        for line in lines
    )


def part_2():
    return ",".join(",".join(list(all_allergens[key])) for key in sorted(all_allergens))


if __name__ == "__main__":
    print(part_1())
    print(part_2())
