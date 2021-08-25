use lander::ship::Ship;

fn main() {
    let mut ship = Ship::new();
    for _ in 0..100 {
        eprintln!("{:?}", ship);
        eprintln!("{:?}", ship.direction());
        ship.integrate();
    }
}
