
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PlayerKind{
    Black,
    White,
}

// impl fmt::Display for PlayerKind {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PlayerKind::Black => write!(f, "Black"),
//             PlayerKind::White => write!(f, "White"),
//         }
//     }
// }
#[derive(Debug)]
pub struct Position{
    pub row:i32,
    pub column: i32,
}

