

    pub trait Hash {
        fn hash(&self) -> usize;
    }

    impl Hash for String {
        fn hash(&self) -> usize {
            let mut sum: usize = 0;
            let chars: Vec<u8> = self.bytes().collect();
            for c in chars {
                sum = sum.overflowing_mul(31).0 + c as usize;
            }
            sum
        }
    }

