#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Block {
    B0,
    B1,
    B2(u8),
    B4,
}
impl TryFrom<char> for Block {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Block> {
        Ok(match c {
            '　' => Block::B0,
            '卒' => Block::B1,
            '关' => Block::B2(0),
            '张' => Block::B2(1),
            '赵' => Block::B2(2),
            '马' => Block::B2(3),
            '黄' => Block::B2(4),
            '曹' => Block::B4,
            _ => anyhow::Result::Err(anyhow::anyhow!("invalid block: {}", c))?,
        })
    }
}
impl From<&Block> for String {
    fn from(block: &Block) -> String {
        use ansi_term::Color::*;
        match block {
            Block::B0    => White.paint("　"),
            Block::B1    => Yellow.bold().reverse().paint("卒"),
            Block::B2(0) => Green.bold().reverse().paint("关"),
            Block::B2(1) => Red.bold().reverse().paint("张"),
            Block::B2(2) => Blue.bold().reverse().paint("赵"),
            Block::B2(3) => Cyan.bold().reverse().paint("马"),
            Block::B2(_) => Purple.bold().reverse().paint("黄"),
            Block::B4    => White.bold().reverse().paint("曹"),
        }.to_string()
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> anyhow::Result<(), std::fmt::Error> {
        let s: String = self.into();
        write!(fmt, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoveOp {
    U, L, D, R
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    state: [[Block; 4]; 5],
}
impl TryFrom<String> for State {
    type Error = anyhow::Error;

    fn try_from(s: String) -> anyhow::Result<State> {
        let mut state = State {
            state: [[Block::B0; 4]; 5],
        };

        let lines = s.trim_end_matches("\n").lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 5, "invalid number of input data lines: {}, expected: 5", lines.len());

        for (y, line) in lines.into_iter().enumerate() {
            let line = line.replace("\x20\x20", "　"); // replace two spaces to one big space
            let chars = line.chars().collect::<Vec<_>>();
            assert_eq!(chars.len(), 4, "invalid input data line: {}", line);

            for (x, c) in chars.into_iter().enumerate() {
                let block = c.try_into()?;
                state.state[y][x] = block;
            }
        }
        Ok(state)
    }
}
impl From<State> for String {
    fn from(state: State) -> String {
        state.state.iter()
            .map(|line_blocks| {
                line_blocks.iter()
                    .map(|block| format!("{}", block))
                    .collect::<Vec<_>>()
                    .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
    }
}
impl std::fmt::Display for State {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> anyhow::Result<(), std::fmt::Error> {
        let s: String = self.clone().into();
        write!(fmt, "{}", s)
    }
}
impl State {
    pub fn get_block(&self, y: usize, x: usize) -> Option<Block> {
        if y <= 4 {
            if x <= 3 {
                return Some(self.state[y][x]);
            }
        }
        return None;
    }

    pub fn find_block0s(&self) -> [(usize, usize); 2] {
        let mut block0s = [(0, 0); 2];
        let mut i = 0;

        'find: for y in 0..5 {
            for x in 0..4 {
                if let Block::B0 = self.state[y][x] {
                    block0s[i] = (y, x);
                    i += 1;
                    if i == 2 {
                        break 'find;
                    }
                }
            }
        }
        return block0s;
    }

    pub fn find_block_rect(&self, y: usize, x: usize) -> [(usize, usize); 2] {
        match self.state[y][x] {
            Block::B0 => return [(y, x), (y, x)],
            Block::B1 => return [(y, x), (y, x)],
            Block::B2(_) => {
                match Some(self.state[y][x]) {
                    b if self.get_block(y - 1, x) == b => return [(y - 1, x), (y, x)],
                    b if self.get_block(y, x - 1) == b => return [(y, x - 1), (y, x)],
                    b if self.get_block(y + 1, x) == b => return [(y, x), (y + 1, x)],
                    b if self.get_block(y, x + 1) == b => return [(y, x), (y, x + 1)],
                    _ => unreachable!(),
                }
            }
            Block::B4 => {
                match Some(self.state[y][x]) {
                    b if self.get_block(y - 1, x - 1) == b => return [(y - 1, x - 1), (y, x)],
                    b if self.get_block(y + 1, x - 1) == b => return [(y, x - 1), (y + 1, x)],
                    b if self.get_block(y - 1, x + 1) == b => return [(y - 1, x), (y, x + 1)],
                    b if self.get_block(y + 1, x + 1) == b => return [(y, x), (y + 1, x + 1)],
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn move_block(&self, y1: usize, x1: usize, y2: usize, x2: usize, op: MoveOp) -> Option<State> {
        match op {
            MoveOp::U => {
                if (x1 ..= x2).all(|x| self.get_block(y1 - 1, x) == Some(Block::B0)) {
                    let mut state = self.clone();

                    for y in y1 ..= y2 {
                        for x in x1 ..= x2 {
                            state.state[y - 1][x] = state.state[y][x];
                        }
                    }
                    for x in x1 ..= x2 {
                        state.state[y2][x] = Block::B0;
                    }
                    return Some(state);
                }
            }
            MoveOp::D => {
                if (x1 ..= x2).all(|x| self.get_block(y2 + 1, x) == Some(Block::B0)) {
                    let mut state = self.clone();

                    for y in (y1 ..= y2).rev() {
                        for x in x1 ..= x2 {
                            state.state[y + 1][x] = state.state[y][x];
                        }
                    }
                    for x in x1 ..= x2 {
                        state.state[y1][x] = Block::B0;
                    }
                    return Some(state);
                }
            }
            MoveOp::L => {
                if (y1..=y2).all(|y| self.get_block(y, x1 - 1) == Some(Block::B0)) {
                    let mut state = self.clone();

                    for y in y1 ..= y2 {
                        for x in x1 ..= x2 {
                            state.state[y][x - 1] = state.state[y][x];
                        }
                    }
                    for y in y1 ..= y2 {
                        state.state[y][x2] = Block::B0;
                    }
                    return Some(state);
                }
            }
            MoveOp::R => {
                if (y1..=y2).all(|y| self.get_block(y, x2 + 1) == Some(Block::B0)) {
                    let mut state = self.clone();

                    for y in y1 ..= y2 {
                        for x in (x1 ..= x2).rev() {
                            state.state[y][x + 1] = state.state[y][x];
                        }
                    }
                    for y in y1 ..= y2 {
                        state.state[y][x1] = Block::B0;
                    }
                    return Some(state);
                }
            }
        }
        None
    }
}
