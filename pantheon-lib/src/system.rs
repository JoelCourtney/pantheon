use seed::prelude::Node;
use serde::{de::DeserializeOwned, Serialize};
use std::cell::{Ref, RefCell};
use std::fmt::Display;

/// The interface that all RPG systems implement.
///
/// This allows the pantheon library to wrap the system's UI and behavior
/// in common features like a menu bar. It also automates the process of
/// expanding expanding a minimal character into a full character.
pub trait System: Default + Clone {
    /// The single source of truth for the full state of the character.
    ///
    /// This is the data stored in the filesystem and edited by the player.
    /// It may not be in a usable state for generating the UI character sheet,
    /// which is the reason for conversion to a full Character.
    ///
    /// # Example
    ///
    /// In D&D 5e, you would not store the Athletics modifier, because this can
    /// be calculated from the following:
    ///
    /// 1. Base strength score
    /// 2. Manual proficiency choices
    /// 3. Race
    ///     - Feats chosen
    /// 4. Class information
    ///     - Total level (for proficiency bonus)
    ///     - Classes chosen
    ///     - Strength ASI's
    ///     - Feats chosen
    /// 5. Manual Ability Score overrides
    ///
    /// You need to store the above information anyway. If you also store
    /// the Athletics modifier, you could potentially run into conflict.
    type MinCharacter: Serialize + DeserializeOwned + Default + SetName + Clone;

    /// Expanded character generated from the minimal character.
    ///
    /// In the example for the [System::MinCharacter] type, this expanded
    /// character is where you would either eagerly or lazily calculate the
    /// Athletics modifier.
    type Character: TryFrom<Self::MinCharacter, Error = Self::SystemError>;

    /// A custom error type for errors that occur during either character conversion
    /// or viewing
    type SystemError: Display;

    type State: Default;
    type Message: Clone;

    /// The name of the system. E.g. "D&D 5e"
    const NAME: &'static str;

    fn view(
        state: &Self::State,
        character: Self::Character,
    ) -> CharacterResult<Vec<Node<crate::ui::Message<Self>>>, Self::SystemError>;
}

/// When a new character is first created, the library needs to tell the
/// system the character's name by setting it in the minimal character.
pub trait SetName {
    fn set_name(&mut self, name: String);
}

/// A value computed lazily with a list of operations to perform.
///
/// The [Character] uses a large collection of these; by wrapping the values
/// in [RefCell]s and computing them lazily, this implicitly performs a
/// depth-first search of the dependency graph.
///
/// For example, if an operation for `char.x` depends on the value of `char.y`,
/// the operation will have to call `char.y.evaluate()`. If nothing attempts to
/// access `x` or `y`, neither value will be computed. If `x` is evaluated, it will
/// trigger `y`'s evaluation first. `y` will only be computed once even if both `x`
/// and `y` are used explicitly.
#[allow(clippy::type_complexity)]
pub struct Lazy<T, S: System, E: Display>(RefCell<(T, Vec<(u8, CharacterOperation<T, S, E>)>)>);

impl<T: Default, S: System, E: Display> Lazy<T, S, E> {
    /// Evaluate the value if needed, and return a [Ref] to it.
    pub fn evaluate(&self, character: &S::Character) -> CharacterResult<Ref<T>, E> {
        let mut immutable_borrow = self.0.borrow();
        if !immutable_borrow.1.is_empty() {
            std::mem::drop(immutable_borrow);
            if let Ok(mut mutable_borrow) = self.0.try_borrow_mut() {
                let (value, ops) = &mut *mutable_borrow;
                ops.sort_unstable_by(|(first_rank, _), (second_rank, _)| {
                    first_rank.cmp(second_rank)
                });
                for (_, op) in ops.drain(..) {
                    op(value, character)?;
                }
            } else {
                return Err(CharacterError::Deadlock);
            }
            immutable_borrow = self.0.borrow();
        }
        Ok(Ref::map(immutable_borrow, |(value, _)| value))
    }

    /// Registers an operation to be performed later.
    pub fn register(&self, rank: u8, op: CharacterOperation<T, S, E>) {
        self.0.borrow_mut().1.push((rank, op));
    }
}

impl<T: Default, S: System, E: Display> Default for Lazy<T, S, E> {
    fn default() -> Self {
        Lazy(RefCell::new((T::default(), Vec::new())))
    }
}

/// An operation performed on the character by a piece of content.
type CharacterOperation<T, S, E> =
    Box<dyn FnOnce(&mut T, &<S as System>::Character) -> CharacterResult<(), E>>;

/// Special result for character operations.
pub type CharacterResult<T, E> = Result<T, CharacterError<E>>;

/// Possible errors during character evaluation.
#[derive(Debug)]
pub enum CharacterError<E: Display> {
    /// Occurs when the engine encounters a cyclical dependency between values.
    ///
    /// Detected by the [RefCell] inside [LazyValue].
    Deadlock,

    /// Allows systems to create their own errors.
    SystemError(E),
}
