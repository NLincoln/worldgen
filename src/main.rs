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
        match self.map.get(((pos.y * self.size.x) + pos.x) as usize) {
            Some(val) => val,
            None => panic!("Position provided to Grid::at that is out of bounds")
        }
    }

    fn assign(&mut self, pos: Vector2u, val: T) {
        if pos.x > self.size.x || pos.y > self.size.y {
            panic!("Position provided to Grid::assign that is out of bounds");
        }
        self.map[((pos.y * self.size.x) + pos.x) as usize] = val;
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

fn carve_room<T, F>(grid: &mut Grid<T>, position: Vector2u, size: Vector2u, f: F) where F: Fn(&Grid<T>, Vector2u) -> T {
    let get_pos = |x: u32, y: u32| vec2u!(position.x + x, position.y + y);

    for y in 0..size.y {
        for x in 0..size.x {
            grid.assign(get_pos(x, y), f(&grid, get_pos(x, y)));
        }
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
    let world = World::new(vec2u!(100, 100));
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
    fn test_carve_grid() {
        let mut grid = create_grid_uint();
        carve_room(&mut grid, vec2u!(1, 1), vec2u!(2, 2), |grid, pos| (*grid.at(pos) * 0));
        assert_eq!(*grid.at(vec2u!(1, 1)), 0);
        assert_eq!(*grid.at(vec2u!(0, 1)), 4);
        carve_room(&mut grid, vec2u!(0, 0), vec2u!(4, 4), |grid, pos| (*grid.at(pos) + 1));
        assert_eq!(*grid.at(vec2u!(0, 0)), 1);

    }

    #[test]
    fn grid_slice_method_works() {
        let grid = create_grid_uint();
        let slice = grid.get_slice(Vector2u::new(1, 1), Vector2u::new(2, 2));
        assert_eq!(*slice.at(vec2u!(0, 0)), grid.at(vec2u!(1, 1)));
        assert_eq!(*slice.at(vec2u!(1, 0)), grid.at(vec2u!(2, 1)));
    }

    #[test]
    fn grid_assign_method() {
        let mut grid = create_grid_uint();
        let pos = vec2u!(1, 1);
        grid.assign(pos, 4);
        assert_eq!(*grid.at(pos), 4);
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