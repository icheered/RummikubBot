use color_eyre::{eyre::Report, eyre::Result};
use rand::Rng;

struct Grid {
    grid: [[u8; 4]; 13],
}

impl Grid {
    fn new(num: u8) -> Grid {
        Grid {
            grid: [[num; 4]; 13],
        }
    }

    fn print(&self) {
        // Print colors Red, Blue, Yellow, Black
        println!("   |  Red | Blue | Yellow | Black");

        for (index, row) in self.grid.iter().enumerate() {
            println!(
                "{:2}    {:2}  | {:2}   | {:2}     | {:2}",
                index + 1,
                row[0],
                row[1],
                row[2],
                row[3]
            );
        }
    }
}

fn grab_tile(bag: &mut Grid, player: &mut Grid) {
    let mut rng = rand::thread_rng();
    let mut color = rng.gen_range(0..4);
    let mut number = rng.gen_range(1..14);

    // Check if the tile is already on the board
    while player.grid[number as usize - 1][color as usize] == 1 {
        color = rng.gen_range(0..4);
        number = rng.gen_range(1..14);
    }

    // Place the tile on the board
    player.grid[number as usize - 1][color as usize] = 1;
    bag.grid[number as usize - 1][color as usize] = 1;
}

pub fn solve() -> Result<(), Report> {
    // Create player grid, a 14x4 grid of 0s
    let mut player = Grid::new(0);
    let mut board = Grid::new(0);
    let mut bag = Grid::new(2);

    // Grab 14 random tiles from the bag and place them on the player
    for tile in 0..14 {
        grab_tile(&mut bag, &mut player);
    }
    player.print();

    Ok(())
}
