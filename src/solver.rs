use color_eyre::{eyre::Report, eyre::Result};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

type Memo = HashMap<u64, Option<Vec<Set>>>;

#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]

struct Tile {
    color: u8,
    number: u8,
    is_joker: bool,
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
            jokers: num,
        }
    }

    fn is_empty(&self) -> bool {
        self.grid.iter().flatten().all(|&tile| tile == 0)
    }

    fn total_tile_count(&self) -> u32 {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .map(|&x| x as u32)
            .sum::<u32>()
            + self.jokers as u32
    }

    fn available_tiles(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(number, row)| {
                row.iter().enumerate().filter_map(move |(color, &count)| {
                    if count > 0 {
                        Some((number, color))
                    } else {
                        None
                    }
                })
            })
            .collect()
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

fn try_form_set_incl_jokers(inventory: &Inventory, number: u8) -> Option<Set> {
    let mut set_tiles: Vec<Tile> = Vec::new();
    let mut jokers_used = 0;

    // Iterate over each color
    for color in 0..4 {
        if inventory.grid[number as usize - 1][color] > 0 {
            set_tiles.push(Tile {
                color: color as u8,
                number,
                is_joker: false,
            });
        } else if jokers_used < inventory.jokers {
            // Use a joker if a tile of the required color is not available
            set_tiles.push(Tile {
                color: color as u8,
                number,
                is_joker: true,
            }); // Mark the tile as a joker
            jokers_used += 1;
        }

        // Check if we have a valid set with 3 tiles
        if set_tiles.len() == 3 {
            return Some(Set { tiles: set_tiles });
        }
    }

    // Return None if we don't have enough tiles (including jokers) for a valid set
    None
}
fn try_form_run_incl_jokers(inventory: &Inventory, start_number: u8, color: u8) -> Option<Set> {
    let mut run_tiles: Vec<Tile> = Vec::new();
    let mut jokers_used = 0;

    // Iterate to check for consecutive numbers with the same color
    for number in start_number..=13 {
        if inventory.grid[number as usize - 1][color as usize] > 0 {
            run_tiles.push(Tile {
                color,
                number,
                is_joker: false,
            });
        } else if jokers_used < inventory.jokers {
            // Use a joker if available
            run_tiles.push(Tile {
                color,
                number,
                is_joker: true,
            }); // Representing the joker
            jokers_used += 1;
        } else {
            break; // Stop if a consecutive number and joker are missing
        }
    }

    // Check if the run has at least 3 tiles
    if run_tiles.len() >= 3 {
        Some(Set { tiles: run_tiles })
    } else {
        None
    }
}

fn grab_tile(source: &mut Inventory, destination: &mut Inventory) {
    let mut rng = rand::thread_rng();
    let total_tiles = source.total_tile_count();
    let grab_joker = source.jokers > 0 && rng.gen_bool(source.jokers as f64 / total_tiles as f64);

    if grab_joker {
        source.jokers -= 1;
        destination.jokers += 1;
    } else {
        if let Some(&(number, color)) = source.available_tiles().choose(&mut rng) {
            source.grid[number as usize][color as usize] -= 1;
            destination.grid[number as usize][color as usize] += 1;
        }
    }
}

fn try_form_set(inventory: &Inventory, number: u8) -> Option<Set> {
    let set_tiles = (0..4)
        .filter_map(|color| {
            if inventory.grid[number as usize - 1][color] > 0 {
                Some(Tile {
                    color: color as u8,
                    number,
                    is_joker: false,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if set_tiles.len() >= 3 {
        Some(Set { tiles: set_tiles })
    } else {
        None
    }
}

fn try_form_run(inventory: &Inventory, start_number: u8, color: u8) -> Option<Set> {
    let run_tiles = (start_number..=13)
        .take_while(|&number| inventory.grid[number as usize - 1][color as usize] > 0)
        .map(|number| Tile {
            color,
            number,
            is_joker: false,
        })
        .collect::<Vec<_>>();

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

    if inventory.is_empty() {
        return Some(Vec::new());
    }

    for number in 1..=13 {
        for color in 0..4 {
            if inventory.grid[number - 1][color] > 0 {
                if let Some(new_set) = try_form_set(inventory, number as u8) {
                    let mut new_inventory = inventory.clone();
                    new_inventory.remove_tiles(&new_set);

                    if let Some(mut solution) = solve_rummikub(&new_inventory, memo) {
                        solution.push(new_set);
                        memo.insert(hash, Some(solution.clone()));
                        return Some(solution);
                    }
                }

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

    memo.insert(hash, None);
    None
}

pub fn solve() -> Result<(), Report> {
    let mut memo = Memo::new();
    let mut player = Inventory::new(0);
    let mut bag = Inventory::new(2);

    solve_rummikub(&player, &mut memo);

    loop {
        grab_tile(&mut bag, &mut player);
        //player.print();
        let solution = solve_rummikub(&player, &mut memo);
        match solution {
            Some(sets) => {
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
