use std::{fmt, ops::Add};

const MATRIX_SIZE: usize = 10;

enum Error {
    CursorOverflow,
    PositionNotEmpty,
}

// DIRECTIONS of moving cursor in a matrix. There are 4 of them: Right, Down, Left and Up.
// Each direction modifies the row or the column of the cursor by 1 unit.
type DirectionOffset = [i32; 2];
const DIRECTIONS: [DirectionOffset; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[derive(Debug)]
struct Direction {
    value: usize,
}

// Directions are represented by 0, 1, 2, 3
impl Direction {
    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn next(&mut self) {
        self.value = (self.value + 1) % 4;
    }
    
    fn offset(&self) -> DirectionOffset {
        DIRECTIONS[self.value]
    }
}

#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
    
    fn set(&mut self, location: &[usize; 2]) {
        self.x = location[0];
        self.y = location[1];
    }
}

impl Add<DirectionOffset> for &Cursor {
    type Output = Result<[usize; 2], Error>;
    fn add(self, rhs: DirectionOffset) -> Self::Output {
        let x = self.x as i32 + rhs[0];
        let y = self.y as i32 + rhs[1];
        if x < 0 || y < 0 || x >= MATRIX_SIZE as i32 || y >= MATRIX_SIZE as i32 {
            return Err(Error::CursorOverflow);
        }
        Ok([x as usize, y as usize])
    }
}

#[derive(Debug)]
struct Matrix {
    matrix: [[usize; MATRIX_SIZE]; MATRIX_SIZE],
    count: usize,
    cursor: Cursor,
    direction: Direction,
}

impl Matrix {
    fn new() -> Self {
        Self {
            matrix: [[0; MATRIX_SIZE]; MATRIX_SIZE],
            count: 0,
            cursor: Cursor::new(),
            direction: Direction::new(),
        }
    }

    fn move_cursor(&mut self) -> Result<(), Error> {
        let location = (&self.cursor + self.direction.offset())?;
        if self.matrix[location[0]][location[1]] != 0 {
            return Err(Error::PositionNotEmpty);
        }
        self.cursor.set(&location);
        Ok(())
    }
    
    fn set_one_position(&mut self) {
        self.matrix[self.cursor.x][self.cursor.y] = self.count;
    }
    
    fn switch_direction(&mut self) {
        self.direction.next();
    }
    
    fn fill_spiral(&mut self) {
        loop {
            self.count += 1;
            self.set_one_position();
            if self.move_cursor().is_err() {
                self.switch_direction();
                if self.move_cursor().is_err() {
                    break;
                }
            }
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let matrix = self.matrix;
        
        for i in matrix {
            for j in i {
                write!(f, "{:4}", j)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut matrix: Matrix = Matrix::new();
    matrix.fill_spiral();
    println!("{}", matrix);
}