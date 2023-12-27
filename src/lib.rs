#![feature(lazy_cell)]

use std::collections::HashMap;
use std::hash::Hash;

pub trait TStateMachine<S, E, A> {
    fn new(states: Vec<S>, events: Vec<E>, transitions: HashMap<(E, S), StateResult<S, A>>) -> Self
    where
        Self: Sized;
    // fn new(states: Vec<S>, events: Vec<E>, transitions: Vec<Vec<StateResult<S, A>>>) -> Self
    // where
    //     Self: Sized;
    fn update_state(&mut self, event: E);
    fn current_state(&self) -> &StateResult<S, A>;
}

#[derive(Clone)]
pub struct StateResult<S, A> {
    pub state: S,
    #[allow(dead_code)]
    pub actions: Vec<A>,
}

#[derive(Clone)]
pub struct StateMachine<S, E, A> {
    state: StateResult<S, A>,
    transitions: HashMap<(E, S), StateResult<S, A>>, //Vec<Vec<StateResult<S, A>>>,
    states: Vec<S>,
    events: Vec<E>,
}

impl<S, E, A> TStateMachine<S, E, A> for StateMachine<S, E, A>
where
    S: Default + PartialEq + Eq + Clone + Hash,
    E: PartialEq + Eq + Clone + Hash,
    A: Clone + Default,
{
    fn new(states: Vec<S>, events: Vec<E>, transitions: HashMap<(E, S), StateResult<S, A>>) -> Self {
        StateMachine { state: StateResult { state: S::default(), actions: Vec::new() }, transitions, states, events }
    }

    fn update_state(&mut self, event: E) {
        println!("UPDATING");
        let alt_state = StateResult { state: S::default(), actions: vec![A::default()] };
        let new_state = self.transitions.get(&(event, self.state.state.clone())).unwrap_or(&alt_state);
        self.state = new_state.clone()
    }
    // fn new(states: Vec<S>, events: Vec<E>, transitions: Vec<Vec<StateResult<S, A>>>) -> Self {
    //     StateMachine { state: StateResult { state: S::default(), actions: Vec::new() }, transitions, states, events }
    // }

    // fn update_state(&mut self, event: E) {
    //     println!("UPDATING");
    //     let ei = self.events.iter().position(|e| e == &event);
    //     let si = self.states.iter().position(|s| s == &self.state.state);
    //     println!("EI {:#?} SI {:#?}", ei, si);
    //     if let (Some(si), Some(ei)) = (si, ei) {
    //         println!("EI2 {} SI2 {}", ei, si);
    //         self.state = self.transitions[ei][si].clone();
    //     }
    // }

    fn current_state(&self) -> &StateResult<S, A> {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    #[derive(Debug, EnumIter, Default, PartialEq, Eq, Clone)]
    enum State {
        #[default]
        Empty,
        InProgress,
        Payed,
        PayDiff,
        Failed,
    }

    #[derive(Debug, EnumIter, Clone)]
    enum Action {
        AddItem,
        DeleteItem,
        Pay,
        RefundDiff,
    }
    #[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
    enum Event {
        ItemAdded,
        ItemDeleted,
        OrderPayed,
    }

    static TRANSITIONS: LazyLock<Vec<Vec<StateResult<State, Action>>>> = LazyLock::new(|| {
        vec![
            vec![
                /* ItemAdded */
                StateResult { state: State::InProgress, actions: vec![Action::AddItem, Action::DeleteItem] }, //Empty
                StateResult { state: State::InProgress, actions: vec![Action::AddItem, Action::DeleteItem] }, //InProgress
                StateResult { state: State::PayDiff, actions: vec![Action::Pay] },                            //Payed
            ],
            vec![
                /* ItemDeleted */
                StateResult { state: State::Failed, actions: vec![Action::AddItem, Action::DeleteItem] }, //Empty
                StateResult { state: State::InProgress, actions: vec![] },                                //InProgress
                StateResult { state: State::Payed, actions: vec![Action::RefundDiff] },                   //Payed
            ],
            vec![
                /* OrderPayed */
                StateResult { state: State::Failed, actions: vec![] }, //Empty
                StateResult { state: State::Payed, actions: vec![] },  //InProgress
                StateResult { state: State::Failed, actions: vec![] }, //Payed
            ],
        ]
    });
    #[test]
    fn test_machine1() {
        let fsm = StateMachine::new(State::iter().collect(), Event::iter().collect(), TRANSITIONS.to_owned());
        assert_eq!(fsm.current_state().state, State::Empty)
    }
    #[test]
    fn test_machine2() {
        let mut fsm = StateMachine::new(State::iter().collect(), Event::iter().collect(), TRANSITIONS.to_owned());
        fsm.update_state(Event::ItemAdded);
        fsm.update_state(Event::ItemAdded);
        fsm.update_state(Event::OrderPayed);
        assert_eq!(fsm.current_state().state, State::Payed);
        assert!(fsm.current_state().actions.is_empty())
    }
}
