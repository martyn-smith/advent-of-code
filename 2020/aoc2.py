"""
Advent of code day 2: "validating" passwords.
"""
import re

with open("2.txt") as f:
    lines = f.readlines()

policy_pattern = re.compile('(\d+)-(\d+) ([a-z]):')

def validate_policy_1(line: str) -> bool:
    """
    character-count based password validation.
    """
    policy = policy_pattern.search(line).groups()
    min_reps = int(policy[0])
    max_reps = int(policy[1])
    char = policy[2]
    password = line.split(":")[1]
    char_count = password.count(char) 
    return (char_count >= min_reps and char_count <= max_reps)

def validate_policy_2(line: str) -> bool:
    """
    character-position based password validation. Exactly one character must be in position a or b.
    """
    policy = policy_pattern.search(line).groups()
    pos_a = int(policy[0])
    pos_b = int(policy[1])
    char = policy[2]
    password = line.split(":")[1]
    return bool(password[pos_a] == char) ^ bool(password[pos_b] == char)

#part_1
print(sum(1 for line in lines if validate_policy_1(line)))

#part_2
with open("2.txt") as f:
    print(sum(1 for line in lines if validate_policy_2(line)))
