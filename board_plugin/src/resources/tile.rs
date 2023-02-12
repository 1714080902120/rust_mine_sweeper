// tile.rs
#[cfg(feature = "debug")]
use colored::Colorize;

/// Enum describing a Minesweeper tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    /// Is a bomb
    Bomb,
    /// Is a bomb neighbor
    BombNeighbor(u8),
    /// Empty tile
    Empty,
}

impl Tile {
    /// Is the tile a bomb?
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
} 