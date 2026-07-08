
struct Ctx {
    counter: i32
}

enum Event {
    OrderBookUpdate
}

enum Action {
    OrderUpdate
}

fn update_event(mut ctx: Ctx, event: &Event) -> (Ctx, Action) {
    match event {
        Event::OrderBookUpdate => {
            ctx.counter += 1;
            return (ctx, Action::OrderUpdate)
        }
        // _: panic!("Unhandled event");
    }
}

fn main() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    let maybe_some_len = maybe_some_string.as_ref().map(|s| s.len());

    println!("maybe_some_len: {:?}", maybe_some_len);
    println!("maybe_some_str: {:?}", maybe_some_string);
}