extern crate rand;

use rand::{Rng,ThreadRng};

#[derive(Debug)]
struct Vector2<T> {
    x: T,
    y: T
}

impl<T> Vector2<T> {
    fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x: x,
            y: y
        }
    }
}

type Vector2u = Vector2<u32>;

struct Tile {
    is_filled: bool
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let val = match self.is_filled {
            true => 'X',
            false => '_'
        };
        write!(f, "{}", val)
    }
}

#[derive(Debug)]
struct World {
    grid: Grid<Tile>
}

struct Grid<T> {
    size: Vector2u,
    map: std::vec::Vec<T>
}

impl<T> Grid<T> {
    fn at(&self, pos: Vector2u) -> &T {
        if pos.x > self.size.x || pos.y > self.size.y {
            panic!("Position provided to World::at that is out of bounds");
        }
        &self.map[((pos.y * self.size.x) + pos.x) as usize]
    }
}

impl<T> std::fmt::Debug for Grid<T> where T: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if let Err(e) = write!(f, "{:?}", self.at(Vector2u::new(x, y))) {
                    return Err(e);
                }
            }
            print!("\n");
        }
        return Ok(());
    }
}


impl World {
    fn new(size: Vector2u) -> World {
        fn create_tile(rng: &mut ThreadRng) -> Tile {
            Tile {
                is_filled: rng.gen::<bool>()
            }
        }

        let mut map_vector = std::vec::Vec::<Tile>::new();
        let mut rng = rand::thread_rng();
        map_vector.reserve_exact((size.x * size.y) as usize);
        for _ in 0..size.x {
            for _ in 0..size.y {
                map_vector.push(create_tile(&mut rng));
            }
        }
        World {
            grid: Grid {
                size: size,
                map: map_vector
            }
        }
    }
}

fn main() {
    let world = World::new(Vector2u::new(10, 10));
    println!("{:?}", world.grid);
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_grid_uint() -> Grid<u32> {
        Grid {
            size: Vector2u {
                x: 4,
                y: 2
            },
            map: vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        }
    }

    #[test]
    fn grid_at_method_works() {
        let grid = create_grid_uint();
        assert_eq!(*grid.at(Vector2u::new(1, 1)), 5);
        assert_eq!(*grid.at(Vector2u::new(0, 0)), 0);
        assert_eq!(*grid.at(Vector2u::new(4, 0)), 4);
    }

    #[test]
    #[should_panic]
    fn world_at_panics_on_out_of_bounds() {
        let grid = create_grid_uint();
        grid.at(Vector2u::new(5, 2));
    }
}