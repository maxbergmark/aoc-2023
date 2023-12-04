pub trait ToBaseTen {
    fn to_base_10(self) -> Option<i32>;
}

impl ToBaseTen for char {
    fn to_base_10(self) -> Option<i32> {
        self.to_digit(10).map(|i| i as i32)
    }
}
