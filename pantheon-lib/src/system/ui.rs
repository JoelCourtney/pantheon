use super::System;

pub struct State<S: System> {
    system_state: S::State
}

pub fn run<S: System>() {

}
