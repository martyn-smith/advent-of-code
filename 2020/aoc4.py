"""
Advent of code day 4: "assisting" passport validation.
"""
import re
re.DOTALL = True

#validations
# byr (Birth Year)
byr = re.compile(".*byr:(\d{4}).*") #1920 - 2002
# iyr (Issue Year)
iyr = re.compile(".*iyr:(\d{4}).*") #2020 - 2020
# eyr (Expiration Year)
eyr = re.compile(".*eyr:(\d{4}).*") #2020 - 2030
# hgt (Height)
hgt = re.compile(".*hgt:(\d+)(cm|in).*") #150-193 cm, 59 - 76 in
# hcl (Hair Color)
hcl = re.compile(".*hcl:\#[0-9a-f]{6}.*")
# ecl (Eye Color)
ecl = re.compile(".*ecl:(amb|blu|brn|gry|grn|hzl|oth).*")
# pid (Passport ID)
pid = re.compile(".*pid:\d{9}.*")
# cid (Country ID) unused
validation = [byr, iyr, eyr, hgt, hcl, ecl, pid]

ranges = { 
    "birth_year": range(1920, 2002+1),
    "issue_year": range(2002, 2020+1),
    "expiration_year": range(2020, 2030+1),
    "height" : {"cm": range(150, 193+1), "in": range(59, 76+1)}
}

with open("4.txt") as f:
    passports = f.read().split("\n\n")

def is_complete(passport: str) -> bool:
    print("matching...")
    for i, v in zip(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"], validation):
        if v.match(passport):
            print(f"{passport} matches {i}")
        else:
            print(f"{passport} does not match {i}")
    return all(v.match(passport) for v in validation)

def is_valid(passport: str) -> bool:
    if not is_complete(passport):
        return False

    birth_year = int(byr.match(passport).group(1))
    issue_year = int(iyr.match(passport).group(1))
    expiration_year = int(eyr.match(passport).group(1))
    height, units = int(hgt.match(passport).group(1)), hgt.match(passport).group(2)

    return all((
            birth_year in ranges["birth_year"],
            issue_year in ranges["issue_year"],
            expiration_year in ranges["expiration_year"],
            height in ranges["height"][units]))

#279 entries, 213 initially valid, 147 secondary valid
print(sum(1 for passport in passports[0:1] if is_complete(passport)))
#print(sum(1 for passport in passports if is_valid(passport)))
