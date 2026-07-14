use presentation_derive::PresentOutput;

mod presentation {
    pub trait Present<Context> {
        fn present(self, context: &mut Context);
    }
}

use presentation::Present;

#[derive(Default)]
struct Log(Vec<&'static str>);

struct First;
struct Middle;
struct Last;

impl Present<Log> for First {
    fn present(self, context: &mut Log) {
        context.0.push("first");
    }
}

impl Present<Log> for Middle {
    fn present(self, context: &mut Log) {
        context.0.push("middle");
    }
}

impl Present<Log> for Last {
    fn present(self, context: &mut Log) {
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
fn presents_fields_in_attribute_order() {
    let output = Output {
        last: Last,
        first: First,
        middle: Middle,
    };
    let mut log = Log::default();

    output.present(&mut log);

    assert_eq!(log.0, ["first", "middle", "last"]);
}
