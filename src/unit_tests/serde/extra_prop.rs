use crate::types::addon::{ExtraProp, OptionsLimit};
use serde_test::{assert_de_tokens, assert_tokens, Token};

#[test]
fn extra_prop() {
    assert_tokens(
        &vec![
            ExtraProp {
                name: "name".to_owned(),
                is_required: true,
                options: Some(vec!["option".to_owned()]),
                options_limit: OptionsLimit(2),
            },
            ExtraProp {
                name: "name".to_owned(),
                is_required: false,
                options: None,
                options_limit: OptionsLimit(1),
            },
        ],
        &[
            Token::Seq { len: Some(2) },
            Token::Struct {
                name: "ExtraProp",
                len: 4,
            },
            Token::Str("name"),
            Token::Str("name"),
            Token::Str("isRequired"),
            Token::Bool(true),
            Token::Str("options"),
            Token::Some,
            Token::Seq { len: Some(1) },
            Token::Str("option"),
            Token::SeqEnd,
            Token::Str("optionsLimit"),
            Token::NewtypeStruct {
                name: "OptionsLimit",
            },
            Token::U64(2),
            Token::StructEnd,
            Token::Struct {
                name: "ExtraProp",
                len: 4,
            },
            Token::Str("name"),
            Token::Str("name"),
            Token::Str("isRequired"),
            Token::Bool(false),
            Token::Str("options"),
            Token::None,
            Token::Str("optionsLimit"),
            Token::NewtypeStruct {
                name: "OptionsLimit",
            },
            Token::U64(1),
            Token::StructEnd,
            Token::SeqEnd,
        ],
    );
    assert_de_tokens(
        &ExtraProp {
            name: "name".to_owned(),
            is_required: false,
            options: None,
            options_limit: OptionsLimit::default(),
        },
        &[
            Token::Struct {
                name: "ExtraProp",
                len: 1,
            },
            Token::Str("name"),
            Token::Str("name"),
            Token::StructEnd,
        ],
    );
}
