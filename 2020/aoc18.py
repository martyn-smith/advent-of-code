"""
Advent of code day 18: horrible stack-based order of operations.
"""
with open("18.txt") as f:
    lines = f.readlines()

def find_closing_brace(expression) -> int:
    braces_level = 1
    for i, c in enumerate(expression):
        if c == "(":
            braces_level += 1
        if c == ")":
            braces_level -= 1
        if braces_level == 0:
            return i
    return None

def find_operator(expression, operator) -> int:
    braces_level = 0
    for i, c in enumerate(expression):
        if c == "(":
            braces_level += 1
        if c == ")":
            braces_level -= 1
        if c == operator and braces_level == 0:
            return i
        if braces_level < 0:
            return i
    return None

def accumulate(value, operator, operand) -> int:
    if operator == "+":
        return value + operand
    if operator == "*":
        return value * operand

def tokenise(expression: str) -> list:
        l = list(expression.rstrip().replace(" ", ""))
        l = [int(c) if c.isdigit() else c for c in l]
        return l

def solve(expression: list, precedence="equal") -> int:
    if len(expression) == 1:
        return expression[0]
    value, operator, operand = expression[0], expression[1], expression[2]
    if value == "(":
        j = find_closing_brace(expression[1:]) + 1
        operand = solve(expression[1:j], precedence)
        acc = accumulate(0, "+", operand)
        return solve([acc] + expression[j+1:], precedence)
    #careful, order of precedence HERE is also killer.
    if operator == "*" and precedence == "AM":
        j = find_operator(expression[2:], "*")
        if j is not None:
            j += 2
            operand = solve(expression[2:j], precedence)
            acc = accumulate(value, operator, operand)
            return solve([acc] + expression[j:], precedence)
        else: 
            operand = solve(expression[2:], precedence)
            return accumulate(value, operator, operand)
    if operand == "(":
        j = find_closing_brace(expression[3:]) + 3
        operand = solve(expression[3:j], precedence)
        acc = accumulate(value, operator, operand)
        return solve([acc] + expression[j+1:], precedence)
    else:
        acc = accumulate(value, operator, operand)
        return solve([acc] + expression[3:], precedence)
    if value in ("+", "*"):
        raise ValueError(f"{expression} is invalid")

# for line in lines:
#     if line[0] != "#":
#         print("==========================================================")
#         print(solve(tokenise(line), "AM"))

# #part 1
print(sum(solve(tokenise(line)) for line in lines))

#part 2
print(sum(solve(tokenise(line), "AM") for line in lines))