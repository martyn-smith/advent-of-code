import re
re.DOTALL = True

#validations
# byr (Birth Year)
byr = re.compile(".*byr:(\d{4}) ") #1920 - 2002
# iyr (Issue Year)
iyr = re.compile(".*iyr:(\d{4}) ") #2020 - 2020
# eyr (Expiration Year)
eyr = re.compile(".*eyr:(\d{4}) ") #2020 - 2030
# hgt (Height)
hgt = re.compile(".*hgt:(\d+)(cm|in)") #150-193 cm, 59 - 76 in
# hcl (Hair Color)
hcl = re.compile(".*hcl:\#[0-9a-f]{6} ")
# ecl (Eye Color)
ecl = re.compile(".*ecl:(amb|blu|brn|gry|grn|hzl|oth)")
# pid (Passport ID)
pid = re.compile(".*pid:\d{9} ")
# cid (Country ID) unused
validation = [byr, iyr, eyr, hgt, hcl, ecl, pid]

def parse_input():
    with open("4.txt") as f:
        lines = f.readlines()
        entries = []
        entry = ""
        for line in lines:
            if line.strip() == "":
                entries.append(entry)
                entry = ""
            else:
                entry += line.strip("\n")+ " "
        entries.append(entry)
    return entries

def is_valid(entry):
    if not all(v.match(entry) for v in validation):
        return False

    birth_year = int(byr.match(entry).group(1))
    if birth_year not in range(1920, 2002+1):
        return False
    issue_year = int(iyr.match(entry).group(1))
    if issue_year not in range(2002, 2020+1):
        return False
    expiration_year = int(eyr.match(entry).group(1))
    if expiration_year not in range(2020, 2030+1):
        return False
    height, units = int(hgt.match(entry).group(1)), hgt.match(entry).group(2)
    if ((units == "cm" and height not in range(150, 193+1)) or
        (units == "in") and height not in range(59, 76+1)):
        return False
    print(entry)
    return True

entries = parse_input()
#278 entries, 213 initially valid, 147 secondary valid

print(sum(1 for entry in entries if is_valid(entry)))
