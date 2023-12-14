
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PlayerKind{
    Black,
    White,
}

#[derive(Debug)]
pub struct Position{
    pub row:i32,
    pub column: i32,
}

