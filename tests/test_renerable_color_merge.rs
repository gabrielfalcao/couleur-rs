i#[cfg(feature = "tracing")] pub use tracing::{Level, event, instrument, span};
#[cfg(feature = "tracing")] use tracing_subscriber::fmt::writer::EitherWriter;
use winnow::{
    ModalResult,
    Parser,
    ascii::{dec_uint, digit1, float, hex_digit1},
    combinator::{
        alt,
        cut_err,
        delimited,
        eof,
        iterator,
        preceded,
        repeat,
        separated,
        separated_pair,
        seq,
        terminated,
    },
    error::{AddContext, ContextError, ErrMode, ParserError, StrContext},
    prelude::*,
    token::{any, none_of, rest, take, take_while},
};
use {

        assert_eq!(
            parse::<&str, ContextError>("{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"),
            Ok(Node::Array(vec![
                Node::Color("#E83B3B".parse::<crate::Color>().unwrap()),
                Node::Text("Hello".to_string()),
                Node::RenderableColor(
                    RenderableColor::new("#E83B3B".parse::<crate::Color>().unwrap())
                        .with_contrast(Contrast::Web)
                ),
                Node::Text(" World".to_string())
            ]))
        );

        Ok(())
    }
}
