// https://www.codewars.com/kata/515bb423de843ea99400000a/train/rust
pub struct PaginationHelper<T> {
    collection: Vec<T>,
    items_per_page: usize,
}

impl<T> PaginationHelper<T> {
    pub fn new(collection: Vec<T>, items_per_page: usize) -> Self {
        Self {
            collection,
            items_per_page,
        }
    }

    pub fn item_count(&self) -> usize {
        self.collection.len()
    }

    pub fn page_count(&self) -> usize {
        if self.collection.len() % self.items_per_page == 0 {
            self.collection.len() / self.items_per_page
        } else {
            self.collection.len() / self.items_per_page + 1
        }
    }

    pub fn page_item_count(&self, page_index: usize) -> Option<usize> {
        match page_index + 1 {
            n if n < self.page_count() => Some(self.items_per_page),
            n if n == self.page_count() => {
                let remainder = self.collection.len() % self.items_per_page;
                Some(if remainder == 0 {
                    self.items_per_page
                } else {
                    remainder
                })
            }
            _ => None,
        }
    }

    pub fn page_index(&self, item_index: usize) -> Option<usize> {
        if item_index < self.collection.len() {
            Some(item_index / self.items_per_page)
        } else {
            None
        }
    }
}
