use super::WP_Post;

#[allow(non_snake_case)]
impl WP_Post {
    pub fn ID(mut self, id: u64) -> Self {
        self.ID = id;

        self
    }
}
