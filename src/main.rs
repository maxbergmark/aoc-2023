mod common {
    pub mod functions;
    pub mod error;
    pub mod traits;
}

mod day_01 {
    pub mod easy;
    pub mod hard;
}

mod day_02 {
    pub mod common;
    pub mod easy;
    pub mod hard;
}

mod day_03 {
    pub mod common;
    pub mod easy;
    pub mod hard;
}

mod day_04 {
    pub mod common;
    pub mod easy;
    pub mod hard;
}

fn main() {
    println!("day 1 (easy): {}", day_01::easy::solve().unwrap());
    println!("day 1 (hard): {}", day_01::hard::solve().unwrap());

    println!("day 2 (easy): {}", day_02::easy::solve().unwrap());
    println!("day 2 (hard): {}", day_02::hard::solve().unwrap());

    println!("day 3 (easy): {}", day_03::easy::solve().unwrap());
    println!("day 3 (hard): {}", day_03::hard::solve().unwrap());

    println!("day 4 (easy): {}", day_04::easy::solve().unwrap());
    println!("day 4 (hard): {}", day_04::hard::solve().unwrap());
}
