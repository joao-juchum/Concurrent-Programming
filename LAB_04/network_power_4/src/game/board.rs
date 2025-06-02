use std::{
    io::{BufWriter, Write},
    ops::Index,
};

use log::trace;

use crate::{
    HEIGHT, POWER, Play, WIDTH,
    game::{CROSS_SEPARATOR, HORIZONTAL_SEPARATOR, VERTICAL_SEPARATOR},
};

use super::{End, FILLER, Player, play};

fn render_player(p: &Player) -> char {
    match p {
        Player::FIRST => 'o',
        Player::SECOND => 'x',
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
/// Represent a board state
pub struct Board {
    inner: [[Option<Player>; WIDTH]; HEIGHT],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            inner: [[const { None }; WIDTH]; HEIGHT],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyError {
    ColumnFull,
}

impl Index<(usize, usize)> for Board {
    type Output = Option<Player>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[index.0][index.1]
    }
}
impl Board {
    /// Render the board into the buffer
    pub fn render<T: std::io::Write>(&self, buffer_out: &mut BufWriter<T>) {
        let interligne: String = String::from(CROSS_SEPARATOR)
            + &(String::from(HORIZONTAL_SEPARATOR) + &String::from(CROSS_SEPARATOR)).repeat(WIDTH)
            + &String::from("\n");
        let mut inner_buffer: [u8; 4] = Default::default();
        buffer_out.write_all(interligne.as_bytes()).unwrap();
        for line in self.inner.iter().rev() {
            for square in line {
                buffer_out.write_all(VERTICAL_SEPARATOR.as_bytes()).unwrap();
                buffer_out
                    .write_all(match square {
                        Some(p) => {
                            let tmp = render_player(p).encode_utf8(&mut inner_buffer);
                            tmp.as_bytes()
                        }
                        None => FILLER.as_bytes(),
                    })
                    .unwrap();
            }
            buffer_out.write_all(VERTICAL_SEPARATOR.as_bytes()).unwrap();
            buffer_out.write_all("\n".as_bytes()).unwrap();
            buffer_out.write_all(interligne.as_bytes()).unwrap();
        }
        buffer_out.write_all(VERTICAL_SEPARATOR.as_bytes()).unwrap();
        let base_point: u32 = 'a' as u32;
        for idx in 0..WIDTH {
            write!(
                buffer_out,
                "{}",
                char::from_u32(base_point + u32::try_from(idx).unwrap()).unwrap()
            )
            .unwrap();
            buffer_out.write_all(VERTICAL_SEPARATOR.as_bytes()).unwrap();
        }
        buffer_out.write_all("\n".as_bytes()).unwrap();
        buffer_out.flush().unwrap();
    }
    /// Get the new board state after playing the play.
    pub fn apply(&self, p: play::Play) -> Result<(Self, Option<End>), ApplyError> {
        trace!("Applying {p:?}");
        for idx in 0..HEIGHT {
            if self[(idx, p.column())].is_none() {
                let mut new = *self;
                new.inner[idx][p.column()] = Some(p.player());
                let w = new.end();
                return Ok((new, w));
            }
        }
        Err(ApplyError::ColumnFull)
    }
    /// Get the new board state after plaing the list of play.
    pub fn apply_sequence(&self, ps: &[Play]) -> (Self, Option<End>) {
        let mut board = *self;
        let mut end = board.end();
        for p in ps.iter() {
            (board, end) = board.apply(*p).unwrap();
        }
        (board, end)
    }
    /// Get the current ending of the board.
    pub fn end(&self) -> Option<End> {
        for p in [Player::FIRST, Player::SECOND] {
            if self.count_align(POWER, p) > 0 {
                return Some(End::Win { player: p });
            }
        }
        {
            // Stall
            let mut check: bool = true;
            for ligne in 0..HEIGHT {
                for colonne in 0..WIDTH {
                    check &= self[(ligne, colonne)].is_some();
                }
            }
            if check {
                return Some(End::Stall);
            }
        }
        None
    }

    fn count_align(&self, size: usize, p: Player) -> usize {
        let mut cnt = 0;
        // Horizontal
        for ligne in 0..HEIGHT {
            for colonne in 0..(WIDTH - size + 1) {
                let mut check: bool = true;
                for idx in 0..size {
                    check &= self[(ligne, colonne + idx)] == Some(p);
                }
                if check {
                    cnt += 1;
                }
            }
        }
        // Vertical
        for ligne in 0..(HEIGHT - size + 1) {
            for colonne in 0..WIDTH {
                let mut check: bool = true;
                for idx in 0..size {
                    check &= self[(ligne + idx, colonne)] == Some(p);
                }
                if check {
                    cnt += 1;
                }
            }
        }
        // Rising
        for ligne in 0..(HEIGHT - size + 1) {
            for colonne in 0..(WIDTH - size + 1) {
                let mut check: bool = true;
                for idx in 0..size {
                    check &= self[(ligne + idx, colonne + idx)] == Some(p);
                }
                if check {
                    cnt += 1;
                }
            }
        }
        // Falling
        for ligne in size..HEIGHT {
            for colonne in 0..(WIDTH - size + 1) {
                let mut check: bool = true;
                for idx in 0..size {
                    check &= self[(ligne - idx, colonne + idx)] == Some(p);
                }
                if check {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    /// Eval the current positions
    ///
    /// The implemention give an adventage to the player that has the most pawn close to the center of the board.
    pub fn naive_eval(&self) -> f64 {
        let width_mid = WIDTH as f64 / 2.0;
        let height_mid = HEIGHT as f64 / 2.0;
        let mut cnt = 0.0;
        for ligne in 0..HEIGHT {
            for colonne in 0..WIDTH {
                let x = self[(ligne, colonne)];
                if let Some(x) = x {
                    let v = 1.0
                        / ((ligne as f64 - height_mid).abs()
                            + (colonne as f64 - width_mid).abs()
                            + 1.0);
                    match x {
                        Player::FIRST => cnt += v,
                        Player::SECOND => cnt -= v,
                    }
                }
            }
        }
        cnt + 10.0 * self.count_align(3, Player::FIRST) as f64
            - 10.0 * self.count_align(3, Player::SECOND) as f64
    }
    /// Get a list of the legal column and board state possible from this board state.
    pub fn legal_moves(&self, p: Player) -> Vec<(usize, Board, Option<End>)> {
        let mut v = Vec::new();
        for idx in 0..WIDTH {
            let play = Play::try_from((idx, p)).unwrap();
            match self.apply(play) {
                Ok((b, e)) => v.push((idx, b, e)),
                Err(e) => trace!("Can't play {idx} becose {e:?}"),
            }
        }
        v
    }
}
