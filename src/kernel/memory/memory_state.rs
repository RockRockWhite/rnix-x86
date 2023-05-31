pub struct MemoryState {
    pub memory_base: usize,
    pub memory_size: usize,
    pub total_pages: usize,
    pub free_pages: usize,
}

impl MemoryState {
    pub fn get_used_pages(&self) -> usize {
        self.total_pages - self.free_pages
    }
}
