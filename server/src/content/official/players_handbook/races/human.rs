macros::register!(("Human", "Race"));

#[derive(Debug)]
pub struct Human;

impl Modify for Human {
    fn initialize(&self, _: &mut Character) {
        unimplemented!()
    }
    fn modify(&self, _: &mut Character) {
        unimplemented!()
    }
    fn finalize(&self, _: &mut Character) {
        unimplemented!()
    }
}