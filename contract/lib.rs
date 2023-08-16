#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
pub mod adder_contract {
    #[ink(storage)]
    pub struct Adder {
        result: i32,
        a: u64,
        c: u64,
        m: u64,
        seed: u64,
  }

    impl Adder {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { result: 0 , a: 1664525, c: 1013904223, m: 2u64.pow(32), seed: 12345 }
        }

        #[ink(message)]
        pub fn add(&mut self, a: i32, b: i32) {
            self.result = mul::add(a, b);
        }

        #[ink(message)]
        pub fn mult(&mut self, a: i32, b: i32) -> i32 {
            self.result = mul::mul(a,b);
            self.result
        }

        #[ink(message)]
        pub fn div(&mut self, a: i32, b: i32) -> i32 {
            self.result = mul::div(a,b);
            self.result
        }

        #[ink(message)]
        pub fn rem(&mut self, a: i32, b: i32) -> i32 {
            self.result = mul::rem(a,b);
            self.result
        }

        #[ink(message)]
        pub fn rng(&mut self ) -> u64{
            self.seed = (self.a * (self.seed) + (self.c)) % self.m;
            self.seed
        }



        #[ink(message)]
        pub fn get_result(&self) -> i32 {
            self.result
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut rng = rng();
            print(rng)
        }
    }
}
