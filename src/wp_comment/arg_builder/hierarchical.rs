#[derive(Debug)]
pub enum Hierarchy {
    /// Retrieves comment tree to the nth depth.
    Threaded(u8),
    /// 'flat' returns a flat array of found comments plus their children.
    Flat,
    /// leaves out descendants.
    False,
}

#[cfg(test)]
mod tests {}
