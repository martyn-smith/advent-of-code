INPUT_LOW = 271973
INPUT_HIGH = 785961


def part_1(start=INPUT_LOW, stop=INPUT_HIGH):
    candidates = []
    for candidate in range(start, stop):
        adjacent, increasing = False, True
        # yes, I've written digit seps before,  But this isn't the criticial path, so easy way.
        digits = [i for i in str(candidate)]
        for i, d in enumerate(digits[:-1]):
            if int(d) > int(digits[i + 1]):
                increasing = False
                continue
            elif int(d) == int(digits[i + 1]):
                adjacent = True
        if adjacent and increasing:
            candidates.append(candidate)
    return len(candidates)


if __name__ == "__main__":
    print(part_1())
