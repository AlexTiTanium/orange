use state::{create_store, Action, Store};

fn main() {
    let mut store = create_store();

    println!("{:?}", store.state().count);

    store.dispatch(Action::Increment);

    println!("{:?}", store.state().count);

    test_one(&mut store);

    store.dispatch(Action::Increment);

    println!("{:?}", store.state().count);

    test_two(&mut store);

    println!("{:?}", store.state().count);
}

fn test_one(store: &mut Store) {
    println!("test one {:?}", store.state().count);
    store.dispatch(Action::Increment);

    test_inside_one(store);
}

fn test_two(store: &mut Store) {
    println!("test two {:?}", store.state().count);
    store.dispatch(Action::Increment);
}

fn test_inside_one(store: &mut Store) {
    println!("test inside {:?}", store.state().count);
    store.dispatch(Action::Increment);
}
