pub mod permutation {
    pub struct Permutations<T> {
        items: Vec<T>,
        stack: Vec<usize>,
        counter: usize,
        is_first: bool,
    }

    impl<T> Permutations<T> {
        pub fn of(items: Vec<T>) -> Permutations<T>{
            let n = items.len();
            Permutations {
                items,
                stack: vec![0; n],
                counter: 0,
                is_first: true,
            }
        }
    }

    impl<T> Iterator for Permutations<T>
    where T: Clone
    {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            // Heap's Algorithm for permutations of a list.
            // https://en.wikipedia.org/wiki/Heap%27s_algorithm
            
            if self.is_first {
                self.is_first = false;
                return Some(self.items.clone());
            }

            while self.counter <= self.stack[self.counter] {
                self.stack[self.counter] = 0;
                self.counter += 1;

                if self.counter >= self.items.len() {
                    return None;
                }
            }

            if self.counter % 2 == 0 {
                self.items.swap(0, self.counter);
            } else {
                self.items.swap(self.stack[self.counter], self.counter)
            }

            self.stack[self.counter] += 1;
            self.counter = 0;

            Some(self.items.clone())
        }
    }
}
