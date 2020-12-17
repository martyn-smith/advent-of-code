"""
Advent of code day 4: "assisting" passport validation.
"""
import re
#re.DOTALL = True

#validations
# byr (Birth Year)
byr = re.compile(".*byr:(\d{4})") #1920 - 2002
# iyr (Issue Year)
iyr = re.compile(".*iyr:(\d{4})") #2020 - 2020
# eyr (Expiration Year)
eyr = re.compile(".*eyr:(\d{4})") #2020 - 2030
# hgt (Height)
hgt = re.compile(".*hgt:(\d+)(cm|in)") #150-193 cm, 59 - 76 in
# hcl (Hair Color)
hcl = re.compile(".*hcl:\#[0-9a-f]{6}")
# ecl (Eye Color)
ecl = re.compile(".*ecl:(amb|blu|brn|gry|grn|hzl|oth)")
# pid (Passport ID)
pid = re.compile(".*pid:\d{9}( |$)")
# cid (Country ID) unused

ranges = { 
    "birth_year": range(1920, 2002+1),
    "issue_year": range(2002, 2020+1),
    "expiration_year": range(2020, 2030+1),
    "height" : {"cm": range(150, 193+1), "in": range(59, 76+1)}
}

with open("4.txt") as f:
    passports = f.read().split("\n\n")

def is_complete(passport: str) -> bool:
    """
    Basic passport validation - just checks if field in present.
    """
    validation = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    return all(v in passport for v in validation)

def is_valid(passport: str) -> bool:
    """
    Enhanced passport validation checks.
    """
    if not is_complete(passport):
        #print(f"{passport}: not complete")
        return False

    if m := byr.match(passport):
        birth_year = int(m.group(1))
    else:
        #print(f"{passport}: no birth year")
        return False
    if m := iyr.match(passport):
        issue_year = int(m.group(1))
    else:
        #print(f"{passport}: no issue year")
        return False
    if m := eyr.match(passport):
        expiration_year = int(m.group(1))
    else:
        #print(f"{passport}: no exp year")
        return False
    if m := hgt.match(passport):
        height, units = int(m.group(1)), m.group(2)
    else:
        #print(f"{passport}: no height")
        return False

    return all((
            birth_year in ranges["birth_year"],
            issue_year in ranges["issue_year"],
            expiration_year in ranges["expiration_year"],
            height in ranges["height"][units],
            hcl.match(passport),
            ecl.match(passport), 
            pid.match(passport)))

#setup. I don't actually know why the regex fails with intermittent newlines... but easy fix.
passports = [passport.replace("\n", " ") for passport in passports]
#279 entries, 213 initially valid, 147 secondary valid.
#part 1
print(sum(1 for passport in passports if is_complete(passport)))

#part 2
print(sum(1 for passport in passports if is_valid(passport)))
