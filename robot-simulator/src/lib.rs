// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
struct X(i32);

impl Into<i32> for X {
    fn into(self) -> i32 {
        self.0
    }
}

#[derive(Clone, Copy)]
struct Y(i32);
impl Into<i32> for Y {
    fn into(self) -> i32 {
        self.0
    }
}

pub struct Robot {
    x: X,
    y: Y,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x: X(x),
            y: Y(y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => Self {
                direction: Direction::East,
                ..self
            },
            Direction::East => Self {
                direction: Direction::South,
                ..self
            },
            Direction::South => Self {
                direction: Direction::West,
                ..self
            },
            Direction::West => Self {
                direction: Direction::North,
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Self {
                direction: Direction::West,
                ..self
            },
            Direction::East => Self {
                direction: Direction::North,
                ..self
            },
            Direction::South => Self {
                direction: Direction::East,
                ..self
            },
            Direction::West => Self {
                direction: Direction::South,
                ..self
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self {
                y: Y(self.y.0 + 1),
                ..self
            },
            Direction::East => Self {
                x: X(self.x.0 + 1),
                ..self
            },
            Direction::South => Self {
                y: Y(self.y.0 - 1),
                ..self
            },
            Direction::West => Self {
                x: X(self.x.0 - 1),
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        // Here is a little trick
        instructions.chars().fold(self, |robot, ins| match ins {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x.into(), self.y.into())
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
