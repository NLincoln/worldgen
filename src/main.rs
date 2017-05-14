#[derive(Debug)]
struct Vector2<T> {
    x: T,
    y: T
}

type Vector2u = Vector2<u32>;
type Vector2f = Vector2<f32>;

#[derive(Debug)]
struct Tile {
    is_filled: bool
}

#[derive(Debug)]
struct World {
    size: Vector2u,
    map: std::vec::Vec<Tile>
}

impl World {
    fn at(&self, pos: Vector2u) -> &Tile {
        &self.map[((pos.y * self.size.x) + pos.y) as usize]
    }

    fn new(size: Vector2u) -> World {
        unimplemented!();
    }
}

fn main() {
    println!("Hello, world!");
}
