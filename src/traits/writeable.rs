pub trait Writable {
    fn enter_char(&mut self, c: char);
    fn pop_char(&mut self);
}
