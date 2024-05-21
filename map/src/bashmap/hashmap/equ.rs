
    pub trait Equ{
        fn equals(&self, obj: &Self) -> bool;
    }
    impl Equ for String{
        fn equals(&self, obj: &Self) -> bool {
            let chars1: Vec<u8> = self.bytes().collect();
            let chars2: Vec<u8> = obj.bytes().collect();
            if chars1.len() != chars2.len() {
                return false;
            }
            for i in 0..chars1.len(){
                if chars2[i] != chars1[i]{
                    return false;
                }
            }
            true
        }
    }
