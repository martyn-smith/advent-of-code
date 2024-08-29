"""
Advent of code day 11. Ferry seats.... as game of life?
"""

from itertools import product


class Ferry:
    """
    Class to hold the Ferry deck's state.
    """

    def __init__(self, tolerance, visibility):
        with open("data/11.txt") as f:
            self.state = [list(line.strip("\n")) for line in f.readlines()]
        self.dimensions = [len(self.state), len(self.state[0])]
        self.prev_state = []
        self.floor = "."
        self.empty = "L"
        self.occupied = "#"
        self.tolerance = tolerance
        self.visibility = visibility

    def count_occupied(self, i, j) -> int:
        if self.visibility == "immediate":
            cells_to_check = [
                c
                for c in product((i - 1, i, i + 1), (j - 1, j, j + 1))
                if all(i >= 0 for i in c)
                and c[0] < self.dimensions[0]
                and c[1] < self.dimensions[1]
                and c != (i, j)
            ]
        if self.visibility == "distant":
            cells_to_check = []
            try:  # looking up
                cells_to_check.append(
                    (
                        next(
                            ix
                            for ix in range(i - 1, -1, -1)
                            if self.prev_state[ix][j] != self.floor
                        ),
                        j,
                    )
                )
            except StopIteration:
                pass
            try:  # looking left
                cells_to_check.append(
                    (
                        i,
                        next(
                            jx
                            for jx in range(j - 1, -1, -1)
                            if self.prev_state[i][jx] != self.floor
                        ),
                    )
                )
            except StopIteration:
                pass
            try:  # looking down
                cells_to_check.append(
                    (
                        next(
                            ix
                            for ix in range(i + 1, self.dimensions[0])
                            if self.prev_state[ix][j] != self.floor
                        ),
                        j,
                    )
                )
            except StopIteration:
                pass
            try:  # looking right
                cells_to_check.append(
                    (
                        i,
                        next(
                            jx
                            for jx in range(j + 1, self.dimensions[1])
                            if self.prev_state[i][jx] != self.floor
                        ),
                    )
                )
            except StopIteration:
                pass
            try:  # looking up-left
                cells_to_check.append(
                    next(
                        (ix, jx)
                        for ix, jx in zip(range(i - 1, -1, -1), range(j - 1, -1, -1))
                        if self.prev_state[ix][jx] != self.floor
                    )
                )
            except StopIteration:
                pass
            try:  # looking up-right
                cells_to_check.append(
                    next(
                        (ix, jx)
                        for ix, jx in zip(
                            range(i - 1, -1, -1), range(j + 1, self.dimensions[1])
                        )
                        if self.prev_state[ix][jx] != self.floor
                    )
                )
            except StopIteration:
                pass
            try:  # looking down-left
                cells_to_check.append(
                    next(
                        (ix, jx)
                        for ix, jx in zip(
                            range(i + 1, self.dimensions[0]), range(j - 1, -1, -1)
                        )
                        if self.prev_state[ix][jx] != self.floor
                    )
                )
            except StopIteration:
                pass
            try:  # looking down-right
                cells_to_check.append(
                    next(
                        (ix, jx)
                        for ix, jx in zip(
                            range(i + 1, self.dimensions[0]),
                            range(j + 1, self.dimensions[1]),
                        )
                        if self.prev_state[ix][jx] != self.floor
                    )
                )
            except StopIteration:
                pass
        return sum(
            1
            for cell in cells_to_check
            if self.prev_state[cell[0]][cell[1]] == self.occupied
        )

    def count_all_occupied(self) -> int:
        return sum(
            sum(1 for cell in row if cell == self.occupied) for row in self.state
        )

    def update_all(self):
        changed = 0
        self.prev_state = [s.copy() for s in self.state]
        for i in range(self.dimensions[0]):
            for j in range(self.dimensions[1]):
                self.update_cell(i, j)

    def update_cell(self, i, j) -> bool:
        """
        If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
        If a seat is occupied (#) and [tolerance] or more seats adjacent to it are also occupied, the seat becomes empty.
        Otherwise, the seat's state does not change.
        """
        cell = self.state[i][j]
        if cell == self.floor:
            return False
        if cell == self.empty:
            if self.count_occupied(i, j) == 0:
                self.state[i][j] = self.occupied
                return True
        if cell == self.occupied:
            if self.count_occupied(i, j) >= self.tolerance:
                self.state[i][j] = self.empty
                return True


def part_1():
    ferry = Ferry(4, "immediate")
    while ferry.state != ferry.prev_state:
        ferry.update_all()
    return ferry.count_all_occupied()


def part_2():
    ferry = Ferry(5, "distant")
    while ferry.state != ferry.prev_state:
        ferry.update_all()
    return ferry.count_all_occupied()


if __name__ == "__main__":
    print(part_1())
    print(part_2())
