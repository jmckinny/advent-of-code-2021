#[derive(Debug)]
pub struct Board {
    data: Vec<Vec<Square>>,
    score : i32
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
            for j in 0..5 {
                data[i].push(Square {
                    flag: false,
                    val: *iter.next().unwrap(),
                });
            }
        }
        Board { data: data, score: 0}
    }

    pub fn calc_score(&mut self, num: i32){
        let mut result = 0;
        for row in self.data.iter() {
            for sqr in row {
                if !sqr.flag {
                    result += sqr.val;
                }
            }
        }
        self.score = result;
        self.score *= num;
    }

    pub fn is_winner(&self) -> bool{
        fn row_win(row: &[Square]) -> bool{
            let sum = row.iter()
            .fold(0, |acc,x| if x.flag {1+acc} else {acc});
            sum == 5
        }

        for (index,row) in self.data.iter().enumerate(){
            if row_win(row){
                return true;
            }
            let mut col_win = 0;
            for col in 0..5{
                if self.data[index][col].flag{
                    col_win += 1;
                }
            }
            if col_win == 5{
                return  true;
            }

        }
        false
    }

    pub fn mark(&mut self, num:i32){
        for row in self.data.iter_mut(){
            for sqr in row{
                if sqr.val == num{
                    sqr.flag = true;
                }
            }
        }
    }

    pub fn get_score(&self) -> i32{
        self.score
    }
    
    pub fn reset(&mut self){
        for row in self.data.iter_mut(){
            for sqr in row{
                sqr.flag = false;
            }
        }
        self.score = 0;
    }

    
}
