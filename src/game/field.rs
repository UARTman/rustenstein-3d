type GameFieldVec = Vec<Vec<char>>;

pub struct GameField {
    pub(crate) field: GameFieldVec,
    // height: usize,
    // width: usize,
}

impl GameField {
    pub fn new(field: GameFieldVec) -> Self {
        // let (height, width) = (field.len(), field[0].len());
        Self {
            field,
            // height,
            // width,
        }
    }
}

impl Default for GameField {
    fn default() -> Self {
        let field_str = include_str!("./field.txt");
        let field_strvec: Vec<&str> = field_str.trim().split('\n').collect();
        let field: Vec<Vec<char>> = field_strvec.iter().map(|x| x.chars().collect()).collect();
        Self::new(field)
    }
}
