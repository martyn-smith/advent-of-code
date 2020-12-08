import re

new_bag_srch = re.compile("^(\w.* \w.*) bags contain")
contained_bag_srch = re.compile("\d (\w.*? \w.*?) bag")
root_bag_srch = re.compile("no other bags")

bag_colours = {}

with open("7.txt") as f:
    bag_list = f.readlines()

for bag_colour in bag_list:
    new_bag = new_bag_srch.match(bag_colour).group(1)
    root_bag = bool(root_bag_srch.match(bag_colour))
    contained_bags = re.findall(contained_bag_srch, bag_colour)
    bag_colours[new_bag] = contained_bags

deleted_bags = []
for bag in bag_colours:
    to_delete = False
    if bag_colours[bag] == []:
        #print(bag)
        to_delete = True
    else:
        for cag in bag_colours:
            if bag in bag_colours[cag]:
                #to_delete = True
                bag_colours[cag] += bag_colours[bag]
    if to_delete:
        deleted_bags.append(bag)

for b in deleted_bags:
    bag_colours.pop(b)

print(len([b for b in bag_colours if "shiny gold" in bag_colours[b]]))