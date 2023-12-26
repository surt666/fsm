#![feature(lazy_cell)]

use std::fmt::Display;

pub trait TStateMachine<S, E, A> {
    fn new(states: Vec<S>, events: Vec<E>, transitions: Vec<Vec<StateResult<S, A>>>) -> Self
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
    transitions: Vec<Vec<StateResult<S, A>>>,
    states: Vec<S>,
    events: Vec<E>,
}

impl<S, E, A> TStateMachine<S, E, A> for StateMachine<S, E, A>
where
    S: Default + PartialEq + Eq + Clone,
    E: PartialEq + Eq + Clone + Display,
    A: Clone,
{
    fn new(states: Vec<S>, events: Vec<E>, transitions: Vec<Vec<StateResult<S, A>>>) -> Self {
        StateMachine { state: StateResult { state: S::default(), actions: Vec::new() }, transitions, states, events }
    }

    fn update_state(&mut self, event: E) {
        println!("UPDATING");
        let ei = self.events.iter().position(|e| {
            println!("E {} EE {}", e, event);
            e == &event
        });
        let si = self.states.iter().position(|s| s == &self.state.state);
        println!("EI {:#?} SI {:#?}", ei, si);
        if let (Some(si), Some(ei)) = (si, ei) {
            println!("EI {} SI {}", ei, si);
            self.state = self.transitions[ei][si].clone();
        }
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

    /*
    events/state        | Empty      | InProgress | Payed                    | Sent                     | Delivered | PayDiff | DeliveryFailed | Failed |
    ItemAdded           | InProgress | InProgress | PayDiff                  | Failed                   | Failed     | PayDiff  | Failed       | Failed |
    ItemDeleted         | Failed     | InProgress | Payed [Refund]           | Failed                   | Failed     | PayDiff  | Failed       | Failed |
    OrderPayed          | Failed     | Payed      | Failed                   | Failed                   | Failed     | Payed    | Failed       | Failed |
    OrderDetailsAdded   | InProgress | Failed     | Failed                   | Failed     | Failed   | Failed       | Failed |
    OrderSent           | Failed     | Sent       | Failed                   | Failed     | Failed   | Failed       | Failed |
    OrderDelivered      | Failed     | Failed     | Delivered                | Failed     | Failed   | Failed       | Failed |
    OrderDeliveryFailed | Failed     | Failed     | DeliveryFailed [ReSend]  | Failed     | Failed   | Failed       | Failed |
    CustomerAdded       | InProgress | Failed     | Failed                   | Failed     | Failed   | Failed       | Failed |
    */
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
