"""
Advent of code, THE FINAL DAY: breaking diffie-hellman.
"""
from itertools import product

g = 7
p = 20201227

def hunt(pub_key, max_tries = p) -> int:
    """
    Brute-forcer for diffie-hellman key exchange.
    """
    for i in range(max_tries):
        if pow(g, i, p) == pub_key:
            return i
    raise StopIteration(f"no match found for key: {pub_key}")

#setup
with open("data/25.txt") as f:
    lines = f.readlines()
    k_door_pub, k_card_pub = int(lines[0]), int(lines[1])

def final_part():
    k_door_priv = hunt(k_door_pub)
    k_card_priv = hunt(k_card_pub)
    return pow(k_door_pub, k_card_priv, p)

if __name__ == "__main__":
    print(final_part())