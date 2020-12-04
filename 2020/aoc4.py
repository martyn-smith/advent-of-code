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

ranges = { 
    "birth_year": range(1920, 2002+1),
    "issue_year": range(2002, 2020+1),
    "expiration_year": range(2020, 2030+1),
    "height" : {"cm": range(150, 193+1), "in": range(59, 76+1)}
}

def parse_input():
    with open("4.txt") as f:
        entry = ""
        for line in f.readlines():
            if line.strip() == "":
                yield entry
                entry = ""
            else:
                entry += line.strip("\n")+ " "
        yield entry

def is_valid(entry):
    if not all(v.match(entry) for v in validation):
        return False

    birth_year = int(byr.match(entry).group(1))
    issue_year = int(iyr.match(entry).group(1))
    expiration_year = int(eyr.match(entry).group(1))
    height, units = int(hgt.match(entry).group(1)), hgt.match(entry).group(2)

    if not all((
            birth_year in ranges["birth_year"],
            issue_year in ranges["issue_year"],
            expiration_year in ranges["expiration_year"],
            height in ranges["height"][units])):
        return False
    return True

#278 entries, 213 initially valid, 147 secondary valid
print(sum(1 for entry in parse_input() if is_valid(entry)))
