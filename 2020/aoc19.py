import re
from functools import lru_cache

with open("19.txt") as f:
    rules, messages = f.read().split("\n\n")
    rules = rules.split("\n")
    messages = messages.split("\n")

#luckily, all rules fall into these categories.
base_rule = re.compile("(\d+): \"(\w)\"")
compound_rule = re.compile("^(\d+): (\d+) (\d+) \| (\d+) (\d+)$")
sequence_rule = re.compile("^(\d+): (\d+) (\d+)$")
lone_rule = re.compile("^(\d+): (\d+)$")
lone_compound_rule = re.compile("^(\d+): (\d+) \| (\d+)$")
#for part 2
lone_rec_rule = re.compile("^(\d+): (\d+) \| (\d+) (\d+)$")
sequence_rec_rule = re.compile("^(\d+): (\d+) (\d+) \| (\d+) (\d+) (\d+)$")


b_count = sum(1 for r in rules if base_rule.match(r))
c_count = sum(1 for r in rules if compound_rule.match(r))
s_count = sum(1 for r in rules if sequence_rule.match(r))
l_count = sum(1 for r in rules if lone_rule.match(r))
lc_count = sum(1 for r in rules if lone_compound_rule.match(r))
#print(f"base, compound, sequence = {b_count}, {c_count}, {s_count}, {l_count}, {lc_count}")

#@lru_cache(None)
def builder(root="0") -> str:
    """
    Recursively builds a string for a regex.
    (Note: for part 1 lru_cache can be enabled.)
    """
    this_rule = re.compile(f"^{root}:.*")
    rule = next(r for r in rules if this_rule.match(r))
    #print(rule)
    if b := base_rule.match(rule):
        return b.group(2)
    elif lr := lone_rec_rule.match(rule):
        return builder(lr.group(2)) + "+"
    elif l := lone_rule.match(rule):
        return builder(l.group(2))
    elif sr := sequence_rec_rule.match(rule):
        return "(" + "|".join(["((" + builder(sr.group(2)) + "){" + str(i) + "}(" 
                              + builder(sr.group(3)) + "){" + str(i) + "})" 
                             for i in range(1,8)]) + ")"
    elif s := sequence_rule.match(rule):
        return builder(s.group(2)) + builder(s.group(3))
    elif lc := lone_compound_rule.match(rule):
        return "(" + builder(lc.group(2)) + "|" + builder(lc.group(3)) + ")"
    elif c := compound_rule.match(rule):
        return "(" + builder(c.group(2)) + builder(c.group(3)) + "|" + builder(c.group(4)) + builder(c.group(5)) + ")"
    print(f"{rule} matches nothing!")

#part 1
b = "^" + builder() + "$"
total_rule = re.compile(b)
print(sum(1 for m in messages if total_rule.match(m)))

#part 2
#325 apparently
i = rules.index("8: 42")
rules = rules[:i] + ["8: 42 | 42 8"] + rules[i:]
i = rules.index("11: 42 31")
rules = rules[:i] + ["11: 42 31 | 42 11 31"] + rules[1:]
b = "^" + builder() + "$"
total_rule = re.compile(b)
print(sum(1 for m in messages if total_rule.match(m)))

