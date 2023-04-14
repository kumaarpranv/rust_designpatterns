pub struct OddNumbers {
    limit: u32,
    current: u32,
}

impl OddNumbers {
    pub fn new(limit: u32) -> OddNumbers {
        OddNumbers { limit, current: 1 }
    }
}

impl Iterator for OddNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.limit {
            return None;
        }

        let current_number = self.current;
        self.current += 2;
        Some(current_number)
    }
}
