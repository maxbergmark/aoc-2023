use super::error::AocError;

pub trait ToBaseTen {
    fn to_base_10(self) -> Option<i32>;
}

impl ToBaseTen for char {
    fn to_base_10(self) -> Option<i32> {
        self.to_digit(10).map(|i| i as i32)
    }
}

pub trait ToAocError<T, E> {
    fn to_aoc_error(self, error: AocError) -> Result<T, AocError>;
}

impl <T, E> ToAocError<T, E> for Result<T, E> {
    fn to_aoc_error(self, error: AocError) -> Result<T, AocError> {
        self.map_err(|_| error)
    }
}