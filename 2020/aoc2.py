import re

policy_pattern = re.compile('(\d+)-(\d+) ([a-z]):')

def validate_policy_1(line):
    policy = policy_pattern.search(line).groups()
    min_reps = int(policy[0])
    max_reps = int(policy[1])
    char = policy[2]
    password = line.split(":")[1]
    char_count = password.count(char) 
    return (char_count >= min_reps and char_count <= max_reps)

def validate_policy_2(line):
    policy = policy_pattern.search(line).groups()
    pos_1 = int(policy[0])
    pos_2 = int(policy[1])
    char = policy[2]
    password = line.split(":")[1]
    return bool(password[pos_1] == char) ^ bool(password[pos_2] == char)

#part_1
with open("2.txt") as f:
    print(sum(1 for line in f.readlines() if validate_policy_1(line)))

#part_2
with open("2.txt") as f:
    print(sum(1 for line in f.readlines() if validate_policy_2(line)))
        
