use ndarray::{Array,Array2};
use itertools::iproduct;
use std::cmp::max;

#[derive(Clone, Copy, Debug)]
pub struct Player {
    position: usize,
    score: usize
}

struct QPlayer {
    /* columns (first axis!) are scores. rows (second axis!) are positions. */
    p_scores: Array2<usize>
}

struct Die {
    position: Option<usize>,
    rolls: usize
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

   fn q_roll(&self) -> [usize; 7] {
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
       [1, 3, 6, 7, 6, 3, 1]
    }

    fn dq_roll(&self) -> [usize; 1] {
        //test function, simpler to reason about
        [1]
    }
}

impl QPlayer {
    fn from(p: Player) -> Self {
        let mut p_scores = Array::zeros((10,21));
        p_scores[[p.position, 0]] = 1;
        QPlayer {
            p_scores
        }
    }

    fn q_play(&mut self, die: &Die) -> usize {
        let mut wins = 0;
        let mut new_scores = Array::zeros((10,21));
        //10 rows, each representing a position. 21 columns, each a score
        //first element is position (0..10), second is score
        for (p, s) in iproduct!(0..self.p_scores.nrows(),
                                0..self.p_scores.ncols()) {

            let amplitude = self.p_scores[[p,s]];
            if amplitude > 0 {
                //actually play the game
                let outcomes = die.q_roll();
                for (i, next) in outcomes.iter().enumerate() {
                    let position = (p + i + 3) % 10;
                    let score = s + position + 1;
                    if score > 20 {
                        wins += amplitude * next;
                    } else {
                        new_scores[[position,score]] += amplitude * next;
                    }
                }
            }
        }
        self.p_scores = new_scores;
        wins
    }
}

pub fn get_input() -> [Player; 2] {
    [Player::new(4), Player::new(8)]
}

pub fn part_1(input: &[Player; 2]) -> usize {
    let (mut player1, mut player2) = (input[0], input[1]);
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

//>3547956381555 (3.5T)
pub fn part_2(input: &[Player]) -> usize {
    /*
     * Create a vector of players.
     * The vector needs to store the number of games that resulted in that (score, score) pair.
     * For each round, create a new vector based off the previous.
     * Pop completed games and tally
     */
    let (mut player1, mut player2) = (QPlayer::from(input[0]), QPlayer::from(input[1]));
    let die = Die::new();
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    while !(player1.p_scores.sum() == 0 || player1.p_scores.sum() == 0) {
        dbg!(&player1.p_scores);
        p1_wins += player1.q_play(&die);
        p2_wins += player2.q_play(&die);
    }
    max(p1_wins, p2_wins)
}
