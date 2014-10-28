use std::io;

fn main() {
    let input = io::stdin().read_line()
                       .ok()
                       .expect("Failed to read line");
    let direction = look_over_there::choose_direction();
    println!("Got input: {}", input);
    if input.as_slice().trim().to_string() == direction.to_string() {
        println!("Gotcha!");
    } else {
        println!("OK, you win!");
    }
}

mod look_over_there {
    use std::fmt;
    use std::rand;

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl fmt::Show for Direction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Up => write!(f, "{}", "up"),
                Down => write!(f, "{}", "down"),
                Left => write!(f, "{}", "left"),
                Right => write!(f, "{}", "right"),
            }
        }
    }

    impl rand::Rand for Direction {
        fn rand<R: rand::Rng>(rng: &mut R) -> Direction {
            let values = [Up, Down, Left, Right];
            let choice = rng.choose(values);
            match choice {
                Some(dir) => *dir,
                _ => {
                    println!("WTF");
                    Up
                }
            }
        }
    }

    pub fn choose_direction() -> Direction {
        let random_direction = rand::random::<Direction>();
        println!("Chose direction: {}", random_direction);
        random_direction
    }

}
