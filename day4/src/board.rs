#[derive(Debug)]
pub struct Board {
    data: Vec<Vec<Square>>,
    score: i32,
}

#[derive(Debug)]
pub struct Square {
    flag: bool,
    val: i32,
}

impl Board {
    pub fn new(input: &[i32]) -> Self {
        let mut iter = input.iter();
        let mut data: Vec<Vec<Square>> = Vec::new();
        for i in 0..5 {
            data.push(Vec::new());
            for _ in 0..5 {
                data[i].push(Square {
                    flag: false,
                    val: *iter.next().unwrap(),
                });
            }
        }
        Board {
            data,
            score: 0,
        }
    }

    pub fn calc_score(&mut self, num: i32) {
        let mut result = 0;
        for row in self.data.iter() {
            for sqr in row {
                if !sqr.flag {
                    result += sqr.val;
                }
            }
        }
        self.score = result * num;
    }

    pub fn is_winner(&self) -> bool {
        //Check rows
        for row in self.data.iter() {
            if 5 == row
                .iter()
                .fold(0, |acc, x| if x.flag { acc + 1 } else { acc })
            {
                return true;
            }
        }
        //Check cols
        for col in 0..5 {
            let mut sum = 0;
            for index in 0..5 {
                if self.data[index][col].flag {
                    sum += 1;
                }
            }
            if sum == 5 {
                return true;
            }
        }

        false
    }

    pub fn mark(&mut self, num: i32) {
        for row in self.data.iter_mut() {
            for sqr in row {
                if sqr.val == num {
                    sqr.flag = true;
                }
            }
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn reset(&mut self) {
        for row in self.data.iter_mut() {
            for sqr in row {
                sqr.flag = false;
            }
        }
        self.score = 0;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formtted = String::from("");
        for row in self.data.iter() {
            for sqr in row {
                formtted.push_str(format!(" ({}, {}) ", sqr.val, sqr.flag).as_str());
            }
            formtted.push('\n');
        }
        write!(f, "{}", formtted)
    }
}
