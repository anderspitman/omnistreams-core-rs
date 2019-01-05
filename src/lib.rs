pub trait Producer<V> {
    fn on_data(&mut self, callback: V);
    fn request(&self, num_items: u8);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
