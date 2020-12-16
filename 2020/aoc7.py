"""
Advent of code day 7: how many bags can a bag bag?
"""
import re

with open("7.txt") as f:
    lines = f.readlines()

#part 1 regexes
new_bag_srch = re.compile("^(\w.* \w.*) bags contain")
child_bag_srch = re.compile("\d (\w.*? \w.*?) bag")
leaf_bag_srch = re.compile("no other bags")

#part 2 regexes
numerical_child_bag_search = re.compile("(\d \w.*? \w.*?) bag")
numbers_split = re.compile("(\d) (\w.*? \w.*)")

def contains_colour(colour: str) -> int:
    """
    Finds how many root bags contain a bag of target colour, by collapsing a recursive list.
    """
    bags = {}

    for line in lines:
        new_bag = new_bag_srch.match(line).group(1)
        child_bags = re.findall(child_bag_srch, line)
        if child_bags != None:
            bags[new_bag] = child_bags

    for bag_1 in bags:
        for bag_2 in bags:
            if bag_1 in bags[bag_2]:
                bags[bag_2] += bags[bag_1]

    return [b for b in bags if colour in bags[b]]

def bag_numbers(colour: str) -> int:
    """
    Finds how many bags a single bag of target colour contains.
    """
    this_colour_srch = re.compile(f"^{colour} bags contain.*")
    this_bag = next(line for line in lines if this_colour_srch.match(line))
    if leaf_bag_srch.match(this_bag):
        return 1
    bag_colours = [numbers_split.match(b).groups() 
                   for b in re.findall(numerical_child_bag_search, this_bag)]
    return 1 + sum(int(bag[0])*bag_numbers(bag[1]) for bag in bag_colours)

#part 1
print(len(contains_colour("shiny gold")))

#part 2
print(bag_numbers("shiny gold") - 1)