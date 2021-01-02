def check_orbits():
    """
    Identifies the total number of orbits (direct and indirect) specified in a 
    text file with the following spec:

    Each line has the form:
    
    PLAN)SAT

    where "PLAN" is the planet and "SAT" the satellite.
    """
    num_orbits = 0
    with open("data/6.txt") as f:
        orbits = [line.strip("\n").split(")") for line in f.readlines()]
    while len(orbits):
        num_orbits += len(orbits)
        new_orbits = []
        for m_orbit in orbits:
            satellite = m_orbit[1]
            for d_orbit in orbits:
                planet = d_orbit[0]
                if satellite == planet:
                    new_orbits.append(d_orbit)
        orbits = new_orbits
    return num_orbits


if __name__ == "__main__":
    print(check_orbits())