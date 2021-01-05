use super::character::Character;

trait Modifier {
    fn initialize(_: &mut Character) {}
    fn modify(_: &mut Character) {}
    fn finalize(_: &mut Character) {}
}

trait Race: Modifier {}
trait Class: Modifier {}
trait Subclass: Modifier {}
trait Background: Modifier {}
trait Feat: Modifier {}