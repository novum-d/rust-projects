pub trait IntExt {
    fn double(&self) -> Self;
}

impl IntExt for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}
