use rand::{
	prelude::*,
	distributions::Alphanumeric
};
#[derive(Clone, PartialEq)]
#[derive(Default)]
pub struct BuyableCharacter {
    name : String,
	walk_speed : usize,
    pub cost : u32
}
impl BuyableCharacter {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
		let s = rng
			.sample_iter(&Alphanumeric)
			.take(10)
			.collect::<String>();
        Self {
            name : s,
            walk_speed : rng.gen_range(1,8),
            cost : rng.gen_range(10,20)
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_speed(&self) -> usize {
        self.walk_speed
    }
}