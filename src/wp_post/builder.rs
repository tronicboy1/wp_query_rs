use super::WpPost;

#[allow(non_snake_case)]
impl WpPost {
    pub fn ID(mut self, id: u64) -> Self {
        self.ID = id;

        self
    }
}
