extern crate rand;

use rand::{Rng,ThreadRng};

#[derive(Debug,Copy,Clone)]
struct Vector2<T> {
    x: T,
    y: T
}

macro_rules! vec2u {
    ($x: expr, $y: expr) => (
        Vector2u {
            x: $x,
            y: $y
        }
    )
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
    fn get_slice(&self, position: Vector2u, size: Vector2u) -> Grid<&T> {
        let mut grid = Grid {
            size: size,
            map: Vec::new()
        };
        for y in 0..size.y {
            for x in 0..size.x {
                grid.map.push(self.at(vec2u!(position.x + x, position.y + y)));
            }
        }
        return grid;
    }

    fn at(&self, pos: Vector2u) -> &T {
        if pos.x > self.size.x || pos.y > self.size.y {
            panic!("Position provided to Grid::at that is out of bounds");
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
mod tests {
    use super::*;

    fn create_grid_uint() -> Grid<u32> {
        Grid {
            size: Vector2u {
                x: 4,
                y: 4
            },
            map: vec![
                0,  1,   2,  3,
                4,  5,   6,  7,
                8,  9,  10, 11,
                12, 13, 14, 15
            ]
        }
    }

    #[test]
    fn grid_slice_method_works() {
        let grid = create_grid_uint();
        let slice = grid.get_slice(Vector2u::new(1, 1), Vector2u::new(2, 2));
        assert_eq!(*slice.at(vec2u!(0, 0)), grid.at(vec2u!(1, 1)));
        assert_eq!(*slice.at(vec2u!(1, 0)), grid.at(vec2u!(2, 1)));
    }

    #[test]
    fn grid_at_method_works() {
        let grid = create_grid_uint();
        assert_eq!(*grid.at(Vector2u::new(1, 1)), 5);
        assert_eq!(*grid.at(Vector2u::new(0, 0)), 0);
        assert_eq!(*grid.at(Vector2u::new(3, 0)), 3);
    }

    #[test]
    #[should_panic]
    fn world_at_panics_on_out_of_bounds() {
        let grid = create_grid_uint();
        grid.at(Vector2u::new(5, 2));
    }
}