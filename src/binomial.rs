/// https://www.geeksforgeeks.org/binomial-coefficient-dp-9/
pub trait Binomial {
    fn binomial(self, k: Self) -> Self;
}

impl Binomial for u32 {
    fn binomial(self, k: Self) -> Self {
        if k == 0 || self == k {
            1
        } else {
            (self - 1).binomial(k - 1) + (self - 1).binomial(k)
        }
    }
}
