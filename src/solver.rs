use color_eyre::{eyre::Report, eyre::Result};
use rand::Rng;
use std::collections::HashMap;

type Memo = HashMap<u64, Option<Vec<Set>>>;

#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]

struct Tile {
    color: u8,
    number: u8,
}

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
struct Set {
    // Group of 3 or 4 tiles with same number and different colors
    // Or run of 3 or more tiles with same color and consecutive numbers
    tiles: Vec<Tile>,
}

impl Set {
    fn print(&self) {
        // If all tiles have same color, print "Group"
        let colors = ["Red", "Blue", "Yellow", "Black"];
        if self
            .tiles
            .iter()
            .all(|&tile| tile.color == self.tiles[0].color)
        {
            //println!("Group: {:?}", self.tiles);
            println!(
                "Group: {:?}",
                self.tiles
                    .iter()
                    .map(|&tile| format!("{} {}", colors[tile.color as usize], tile.number))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        } else {
            //println!("Run: {:?}", self.tiles);
            println!(
                "Run: {:?}",
                self.tiles
                    .iter()
                    .map(|&tile| format!("{} {}", colors[tile.color as usize], tile.number))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]

struct Inventory {
    grid: [[u8; 4]; 13],
    jokers: u8,
}

impl Inventory {
    fn new(num: u8) -> Inventory {
        Inventory {
            grid: [[num; 4]; 13],
            jokers: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.grid.iter().flatten().all(|&tile| tile == 0) && self.jokers == 0
    }

    fn remove_tiles(&mut self, set: &Set) {
        for tile in &set.tiles {
            self.grid[tile.number as usize - 1][tile.color as usize] -= 1;
        }
    }

    fn hash(&self) -> u64 {
        let mut hash: u64 = 0; // Explicit type declaration for hash
        self.grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &tile)| {
                hash = hash
                    .wrapping_mul(31_u64) // Specify type for 31 as u64
                    .wrapping_add(tile as u64)
                    .wrapping_add(i as u64)
                    .wrapping_add(j as u64);
            });
        });

        // Incorporate jokers into the hash
        hash.wrapping_mul(31_u64).wrapping_add(self.jokers as u64) // Specify type for 31 as u64
    }

    fn print(&self) {
        // Print colors Red, Blue, Yellow, Black
        println!("Jokers: {}", self.jokers);
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

fn grab_tile(bag: &mut Inventory, player: &mut Inventory) {
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

fn try_form_set(inventory: &Inventory, number: u8) -> Option<Set> {
    let mut set_tiles: Vec<Tile> = Vec::new();

    // Iterate over each color
    for color in 0..4 {
        if inventory.grid[number as usize - 1][color] > 0 {
            set_tiles.push(Tile {
                color: color as u8,
                number,
            });

            // Break early if we already have 3 tiles (a valid set)
            if set_tiles.len() == 3 {
                return Some(Set { tiles: set_tiles });
            }
        }
    }

    // Return None if we don't have enough tiles for a valid set
    None
}

fn try_form_run(inventory: &Inventory, start_number: u8, color: u8) -> Option<Set> {
    let mut run_tiles: Vec<Tile> = Vec::new();

    // Iterate to check for consecutive numbers with the same color
    for number in start_number..=13 {
        if inventory.grid[number as usize - 1][color as usize] > 0 {
            run_tiles.push(Tile { color, number });
        } else {
            break; // Stop if a consecutive number is missing
        }
    }

    // Check if the run has at least 3 tiles
    if run_tiles.len() >= 3 {
        Some(Set { tiles: run_tiles })
    } else {
        None
    }
}

fn solve_rummikub(inventory: &Inventory, memo: &mut Memo) -> Option<Vec<Set>> {
    let hash = inventory.hash();

    if let Some(solution) = memo.get(&hash) {
        return solution.clone();
    }

    // Base case: if inventory is empty, return an empty solution
    if inventory.is_empty() {
        return Some(Vec::new());
    }

    // Iterate through tiles in the inventory to form sets or runs
    for number in 1..=13 {
        for color in 0..4 {
            if inventory.grid[number - 1][color] > 0 {
                // Try to form a set with this tile
                if let Some(new_set) = try_form_set(inventory, number as u8) {
                    let mut new_inventory = inventory.clone();
                    new_inventory.remove_tiles(&new_set);

                    if let Some(mut solution) = solve_rummikub(&new_inventory, memo) {
                        solution.push(new_set);
                        memo.insert(hash, Some(solution.clone()));
                        return Some(solution);
                    }
                }

                // Try to form a run with this tile
                if let Some(new_run) = try_form_run(inventory, number as u8, color as u8) {
                    let mut new_inventory = inventory.clone();
                    new_inventory.remove_tiles(&new_run);

                    if let Some(mut solution) = solve_rummikub(&new_inventory, memo) {
                        solution.push(new_run);
                        memo.insert(hash, Some(solution.clone()));
                        return Some(solution);
                    }
                }
            }
        }
    }

    // If no solution is found
    memo.insert(hash, None);
    None
}

pub fn solve() -> Result<(), Report> {
    let mut memo = Memo::new();
    let mut player = Inventory::new(0);
    let mut bag = Inventory::new(2);

    let mut found_solution = false;
    while !found_solution {
        grab_tile(&mut player, &mut bag);
        let solution = solve_rummikub(&player, &mut memo);
        match solution {
            Some(sets) => {
                found_solution = true;
                let num_tiles = player
                    .grid
                    .iter()
                    .flatten()
                    .fold(0, |acc, &tile| acc + tile);
                println!("Solution found after {} tiles", num_tiles);
                for set in sets {
                    set.print();
                }

                player.print();
                break;
            }
            None => {
                // Print number of tiles
                let num_tiles = player
                    .grid
                    .iter()
                    .flatten()
                    .fold(0, |acc, &tile| acc + tile);
                println!("No solution found after {} tiles", num_tiles);
            }
        }
    }
    Ok(())
}
