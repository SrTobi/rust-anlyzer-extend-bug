use super::Test;
use extend::ext;

#[ext(pub)]
impl Test {
    fn test(&self) {
        println!("test");
    }
}