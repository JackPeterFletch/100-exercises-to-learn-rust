// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

#[cfg(test)]
mod tests {
    use super::{Power, PowerAsso};

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_amper_u32() {
        let y: &u32 = &2_u32;

        let x: u32 = y.power_asso(&3u32);
        assert_eq!(x, 8);
    }
}

trait Power<Input> {
    //type Input;


    fn power(&self, input: Input) -> Self;
}

impl Power<u16> for u32 {
    fn power(&self, input: u16) -> Self {
        self.pow(input.into())
    }
}

impl Power<u32> for u32 {
    fn power(&self, input: u32) -> Self {
        self.pow(input.into())
    }
}

impl Power<&u32> for u32 {
    fn power(&self, input: &u32) -> Self {
        self.pow(*input)
    }
}

// with associated type this would be better as it'd work on &u32 as self

trait PowerAsso<Input> {
    type Output;


    fn power_asso(&self, input: Input) -> Self::Output;
}

impl PowerAsso<u16> for u32 {
    type Output = u32;
    fn power_asso(&self, input: u16) -> Self::Output {
        self.pow(input.into())
    }
}

impl PowerAsso<u32> for u32 {
    type Output = u32;
    fn power_asso(&self, input: u32) -> Self::Output {
        self.pow(input.into())
    }
}

impl PowerAsso<&u32> for u32 {
    type Output = u32;
    fn power_asso(&self, input: &u32) -> Self::Output {
        self.pow(*input)
    }
}

impl PowerAsso<&u32> for &u32 {
    type Output = u32;
    fn power_asso(&self, input: &u32) -> Self::Output {
        self.pow(*input)
    }
}