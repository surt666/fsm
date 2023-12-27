#![feature(lazy_cell)]

use std::collections::HashMap;
use std::hash::Hash;

pub trait TStateMachine<S, E, A> {
    fn new(states: Vec<S>, events: Vec<E>, transitions: HashMap<(E, S), StateResult<S, A>>) -> Self
    where
        Self: Sized;
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
    transitions: HashMap<(E, S), StateResult<S, A>>,
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

    fn current_state(&self) -> &StateResult<S, A> {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use strum::IntoEnumIterator;
    use strum_macros::{EnumDiscriminants, EnumIter};

    #[derive(Debug, EnumIter, Default, PartialEq, Eq, Clone, Hash)]
    enum State {
        #[default]
        Empty,
        InProgress,
        Payed,
        PayDiff,
        Failed,
    }

    #[derive(Debug, EnumIter, Clone, Default)]
    enum Action {
        #[default]
        AddItem,
        DeleteItem,
        Pay,
        RefundDiff,
    }
    #[derive(Debug, EnumIter, PartialEq, Eq, Clone, EnumDiscriminants)]
    #[strum_discriminants(derive(EnumIter, Hash))]
    enum Event {
        ItemAdded,
        ItemDeleted,
        OrderPayed,
    }

    static TRANSITIONS: LazyLock<HashMap<(EventDiscriminants, State), StateResult<State, Action>>> = LazyLock::new(|| {
        let mut map: HashMap<(EventDiscriminants, State), StateResult<State, Action>> = HashMap::new();

        /* ItemAdded */
        map.insert(
            (EventDiscriminants::ItemAdded, State::Empty),
            StateResult { state: State::InProgress, actions: vec![Action::AddItem, Action::DeleteItem] },
        );
        map.insert(
            (EventDiscriminants::ItemAdded, State::InProgress),
            StateResult { state: State::InProgress, actions: vec![Action::AddItem, Action::DeleteItem] },
        );
        map.insert((EventDiscriminants::ItemAdded, State::Payed), StateResult { state: State::PayDiff, actions: vec![Action::Pay] });
        /* ItemDeleted */
        map.insert(
            (EventDiscriminants::ItemDeleted, State::Empty),
            StateResult { state: State::Failed, actions: vec![Action::AddItem, Action::DeleteItem] },
        );
        map.insert(
            (EventDiscriminants::ItemDeleted, State::InProgress),
            StateResult { state: State::InProgress, actions: vec![Action::AddItem, Action::DeleteItem] },
        );
        map.insert((EventDiscriminants::ItemDeleted, State::Payed), StateResult { state: State::Payed, actions: vec![Action::RefundDiff] });
        /* OrderPayed */
        map.insert((EventDiscriminants::OrderPayed, State::Empty), StateResult { state: State::Failed, actions: vec![] });
        map.insert((EventDiscriminants::OrderPayed, State::InProgress), StateResult { state: State::Payed, actions: vec![] });
        map.insert((EventDiscriminants::OrderPayed, State::Payed), StateResult { state: State::Failed, actions: vec![] });
        map
    });

    #[test]
    fn test_machine1() {
        let fsm = StateMachine::new(State::iter().collect(), EventDiscriminants::iter().collect(), TRANSITIONS.to_owned());
        assert_eq!(fsm.current_state().state, State::Empty)
    }
    #[test]
    fn test_machine2() {
        let mut fsm = StateMachine::new(State::iter().collect(), EventDiscriminants::iter().collect(), TRANSITIONS.to_owned());
        fsm.update_state(EventDiscriminants::ItemAdded);
        fsm.update_state(EventDiscriminants::ItemAdded);
        fsm.update_state(EventDiscriminants::OrderPayed);
        assert_eq!(fsm.current_state().state, State::Payed);
        assert!(fsm.current_state().actions.is_empty())
    }
}
