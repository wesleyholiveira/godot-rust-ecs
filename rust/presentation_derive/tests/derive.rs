use presentation_derive::PresentOutput;

mod presentation {
    pub trait Present<Context> {
        fn present(&mut self, context: &mut Context);
    }
}

use presentation::Present;

#[derive(Default)]
struct Log(Vec<&'static str>);

struct First;
struct Middle;
struct Last;

impl Present<Log> for First {
    fn present(&mut self, context: &mut Log) {
        context.0.push("first");
    }
}

impl Present<Log> for Middle {
    fn present(&mut self, context: &mut Log) {
        context.0.push("middle");
    }
}

impl Present<Log> for Last {
    fn present(&mut self, context: &mut Log) {
        context.0.push("last");
    }
}

#[derive(PresentOutput)]
struct Output {
    // A ordem física dos campos é deliberadamente diferente da ordem de apply.
    #[present(order = 90)]
    last: Last,

    #[present(order = 10)]
    first: First,

    #[present(order = 40)]
    middle: Middle,
}

#[test]
fn presents_fields_in_attribute_order_and_can_be_reused() {
    let mut output = Output {
        last: Last,
        first: First,
        middle: Middle,
    };
    let mut log = Log::default();

    output.present(&mut log);
    output.present(&mut log);

    assert_eq!(
        log.0,
        ["first", "middle", "last", "first", "middle", "last"]
    );
}
