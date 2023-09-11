pub trait Absolute {
    fn abs(&mut self) -> Self;
}

impl Absolute for f32 {
    fn abs(&mut self) -> Self {
        if self.is_sign_negative() {
            *self *= -1.0;
        }
        *self
    }
}
