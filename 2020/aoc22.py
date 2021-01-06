"""
Advent of code day 22: recursive crab
"""

def make_decks():
    deck_a = [int(c) for c in player_a.split("\n")[1:]]
    deck_b = [int(c) for c in player_b.split("\n")[1:]]
    return deck_a, deck_b

def score(deck):
    return sum((1+i) * j for i, j in enumerate(deck[::-1]))

def play_combat(deck_a, deck_b):
    while deck_a != [] and deck_b != []:
        card_a, card_b = deck_a.pop(0), deck_b.pop(0)
        if card_a > card_b:
            deck_a += [card_a, card_b]
        elif card_b > card_a:
            deck_b += [card_b, card_a]
        else:
            raise ValueError("cards are equal - I don't know what to do.")
    win_deck = next(deck for deck in (deck_a, deck_b) if deck != [])
    return score(win_deck)

def play_recursive_combat(deck_a, deck_b, g = 1):
    """
    Plays the recursive game for part 2.
    Considered a dynamic programming approach, which is why deck_a and deck_b are passed as
    tuples (i.e. hashable types) instead of lists. We could potentially use functools.lru_cache,
    but would need to implement a way to ensure it always returns "a"?
    """
    #print(f"=== Game {g} ===\n")
    prev_decks = []
    deck_a, deck_b = list(deck_a), list(deck_b)
    r = 0
    while deck_a != [] and deck_b != []:
        r += 1
        #print(f"-- Round {r} (Game {g}) --")
        if (deck_a, deck_b) in prev_decks:
            #print(f"I've seen one of these decks before: {deck_a}, {deck_b}")
            return "a"
        #print(f"Player 1's deck: {deck_a}")
        #print(f"Player 2's deck: {deck_b}")
        prev_decks.append((deck_a.copy(),deck_b.copy()))
        card_a, card_b = deck_a.pop(0), deck_b.pop(0)
        #print(f"Player 1 plays: {card_a}")
        #print(f"Player 2 plays: {card_b}")
        
        if len(deck_a) < card_a or len(deck_b) < card_b:
            if card_a == card_b:
                raise ValueError("cards are equal - I don't know what to do.")
            round_winner = ("a" if card_a > card_b else "b")
        #    print(f"Player {round_winner} wins round {r} of game {g}")
        else:
            #recurse
        #    print("playing a subgame to determine the winner...")
            subdeck_a, subdeck_b = deck_a.copy()[:card_a], deck_b.copy()[:card_b]
            round_winner = play_recursive_combat(tuple(subdeck_a), tuple(subdeck_b), g+1)
        #    print(f"the the winner of game {g+1} is {round_winner}!\n\nanyway, black to game {g}")
        if round_winner == "a":
            deck_a += [card_a, card_b]
        elif round_winner == "b":
            deck_b += [card_b, card_a]
        else:
            raise TypeError(f"{round_winner} is unexpected")
    
    game_winner =  "a" if deck_a != [] else "b"
    #print(f"the winner of game {g} is player {game_winner}")
    if g > 1:
        return game_winner
    else:
        return score(deck_b)

#setup
with open("22.txt") as f:
    player_a, player_b = f.read().split("\n\n")

#part 1
deck_a, deck_b = make_decks()
print(play_combat(deck_a, deck_b))

#part 2
deck_a, deck_b = make_decks()
print(play_recursive_combat(tuple(deck_a), tuple(deck_b)))