/*
 * combinations and probabilities for three three-sided die:
 *
 * x  |  P
 * -------
 * 3  |  1
 * 4  |  3
 * 5  |  6
 * 6  |  7
 * 7  |  6
 * 8  |  3
 * 9  |  1
 *
 */

#[derive(Clone, Debug)]
pub struct Player {
    position: usize,
    score: usize
}

struct QPlayer {
    scores: Vec<usize>
}

struct Die {
    position: Option<usize>,
    rolls: usize
}


struct Game {
    player1: Player,
    player2: Player
}

impl Player {
    fn new(position: usize) -> Self {
        Player {
            position: position - 1,
            score: 0
        }
    }

    fn play(&mut self, die: &mut Die) -> bool {
        let x = (0..3).map(|_| die.roll()).sum::<usize>();
        self.position = (self.position + x) % 10;
        self.score += self.position + 1;
        self.score + 1 >= 1000
    }
}

impl Die {
    fn new() -> Self {
        Die {
            position: None,
            rolls: 0
        }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        if self.position.is_none() {
            self.position = Some(0);
        } else {
            self.position = Some((self.position.unwrap() + 1) % 100);
        }
        self.position.unwrap() + 1
    }

    fn q_roll(&mut self) -> [usize; 7] {
        //self.rolls += 1;
        //number of possibilities of 3..=9
        [1, 3, 6, 7, 6, 3, 1]
    }
}

pub fn get_input() -> [Player; 2] {
    [Player::new(10), Player::new(1)]
}

pub fn part_1(input: &[Player; 2]) -> usize {
    let (mut player1, mut player2) = (input[0].clone(), input[1].clone());
    let mut die = Die::new();
    loop {
        if player1.play(&mut die) {
            return player2.score * die.rolls;
        }
        if player2.play(&mut die) {
            return player1.score * die.rolls;
        }
    }
}

pub fn part_2(input: &[Player]) -> usize {
    /*
     * Create a vector of players.
     * The vector needs to store the number of games that resulted in that (score, score) pair.
     * For each round, create a new vector based off the previous.
     * Pop completed games and tally
     */
    let (mut player1, mut player2) = (input[0].clone(), input[1].clone());
    let mut die = Die::new();
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    // let mut p1_scores = vec![];
    // let mut p2_scores = vec![];
    0
}
