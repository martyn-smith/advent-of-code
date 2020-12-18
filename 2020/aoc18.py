"""
Advent of code day 18: implementing a tokenizer.
"""
def find_matching_close(expression_string) -> int:
    """
    Self-explanatory, although it expects to given the subexpression_string immediately after the open bracket:
    '( foo)' -> ' foo)'. This has some neat behaviour for part 2.
    """
    braces_depth = 1
    for i, c in enumerate(expression_string):
        if c == "(":
            braces_depth += 1
        if c == ")":
            braces_depth -= 1
        if braces_depth == 0:
            return i + 1

def accumulate(accumulator: int, op: str, c: int) -> int:
    """
    Does the accumulation.
    """
    #print(f"{accumulator} {op} {c}")
    if op == "+":
        return accumulator + c
    if op == "*":
        return accumulator * c

def accumulate_expression(expression_string: str, precedence="equal") -> int:
    """
    Traverses a expression_string and evaluates according to strict left- precedence rules.
    """
    op = ""
    accumulator = 0
    i = 0
    expression_string = list(expression_string)
    if expression_string[0] == "(":
        expression_string = ["0", " ",  "+", " "] + expression_string
    #print(f"parsing... {expression_string}")
    while i in range(len(expression_string)):
        c = expression_string[i]
        if c == " ":
            pass
        if c == "+":
            op = c
        if c == "*":
            op = c
            if precedence == "AM":
                j = find_matching_close(expression_string[i+2:])
                if j is not None:
                    j += i
                    expression_string = expression_string[:i+1] + ["("] + expression_string[i+2:j] + [")"] + expression_string[j+2:]
                else:
                    expression_string = expression_string[:i+1] + ["("] + expression_string[i+2:] + [")"]
        if c.isdigit():
            if op == "":
                if accumulator == 0:
                    #print(f"initialising with {c}")
                    accumulator = int(c)
                else:
                    print("no operation! quitting...")
                    exit(1)
            else:
                accumulator = accumulate(accumulator, op, int(c))
        if c == "(":
            j = i + find_matching_close(expression_string[i+1:])
            expression_string = (expression_string[:i]
             + [accumulate_expression(expression_string[i+1:j], precedence)] 
             + expression_string[j+1:])
            accumulator = accumulate(accumulator, op, expression_string[i])
        i += 1
    #print(f"returning {accumulator}")
    return accumulator

with open("18.txt") as f:
    lines = f.readlines()

#part 1
print(sum(accumulate_expression(line) for line in lines))

#part 2
print(sum(accumulate_expression(line, "AM") for line in lines))
