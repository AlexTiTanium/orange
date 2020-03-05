#[allow(dead_code)]
mod resources;
use state::{create_store, Action, Store};

fn main() {
    let mut store = create_store();

    store.dispatch(Action::Increment);
    println!("{:?}", store.state().count);
    test(&mut store);
    //println!("{:?}", resources::SHADER_BUM_VERT);
}

fn test(store: &mut Store) {
    store.dispatch(Action::Increment);
    println!("{:?}", store.state().count);
}
