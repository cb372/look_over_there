

fn main() {
  println!("{}", look_over_there::choose_direction());
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
            Up
        }
    }

    pub fn choose_direction() -> Direction {
        let random_direction = rand::random::<Direction>();
        println!("Chose direction: {}", random_direction);
        random_direction
    }

}
