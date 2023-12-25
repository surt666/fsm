/*
events/state        | InProgress | Payed    | Sent                     | Delivered | PayDiff | DeliveryFailed | Failed |
ItemAdded           | InProgress | PayDiff  | Failed                   | Failed     | PayDiff  | Failed       | Failed |
ItemDeleted         | InProgress | PayDiff  | Failed                   | Failed     | PayDiff  | Failed       | Failed |
OrderPayed          | Payed      | Failed   | Failed                   | Failed     | Payed    | Failed       | Failed |
OrderDetailsAdded   | InProgress | Failed   | Failed                   | Failed     | Failed   | Failed       | Failed |
OrderSent           | Failed     | Sent     | Failed                   | Failed     | Failed   | Failed       | Failed |
OrderDelivered      | Failed     | Failed   | Delivered                | Failed     | Failed   | Failed       | Failed |
OrderDeliveryFailed | Failed     | Failed   | DeliveryFailed [ReSend]  | Failed     | Failed   | Failed       | Failed |
CustomerAdded       | InProgress | Failed   | Failed                   | Failed     | Failed   | Failed       | Failed |
*/

pub trait StateMachine<S, E, A> {
  fn new(states: Vec<S>, events: Vec<E>, transitions: Vec<Vec<StateResult<S, A>>>) -> Self
  where
    Self: Sized;
  fn update_state(&self, event: E);
  fn current_state(&self) -> &StateResult<S, A>;
}

#[derive(Clone)]
struct StateResult<S, A> {
  state: S,
  actions: Vec<A>,
}
#[derive(Clone)]
struct MyStateMachine<S, E, A> {
  state: StateResult<S, A>,
  transitions: Vec<Vec<StateResult<S, A>>>,
  states: Vec<S>,
  events: Vec<E>,
}

impl<S, E, A> StateMachine<S, E, A> for MyStateMachine<S, E, A>
where
  S: Default,
{
  fn new(states: Vec<S>, events: Vec<E>, transitions: Vec<Vec<StateResult<S, A>>>) -> Self {
    MyStateMachine { state: StateResult { state: S::default(), actions: Vec::new() }, transitions, states, events }
  }

  fn update_state(&self, event: E) {}

  fn current_state(&self) -> &StateResult<S, A> {
    &self.state
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use strum::IntoEnumIterator;
  use strum_macros::EnumIter;

  #[derive(Debug, EnumIter, Default, PartialEq, Eq)]
  enum State {
    #[default]
    Empty,
    InProgress,
    Payed,
  }

  #[derive(Debug, EnumIter)]
  enum Action {
    AddItem,
    DeleteItem,
    Pay,
  }
  #[derive(Debug, EnumIter)]
  enum Event {
    ItemAdded,
    ItemDeleted,
    Payed,
  }

  #[test]
  fn test_machine() {
    let fsm = MyStateMachine::new(
      State::iter().collect(),
      Event::iter().collect(),
      vec![vec![StateResult { state: State::InProgress, actions: vec![Action::AddItem] }]],
    );
    assert_eq!(fsm.current_state().state, State::Empty)
  }
}
