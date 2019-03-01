use std::collections::HashMap;

// use serde::Deserialize;

use hocon::{Hocon, HoconLoader};

#[test]
fn parse_string() {
    let s = r#"{"a":"dndjf"}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_string().unwrap(), "dndjf");
}

#[test]
fn parse_int() {
    let s = r#"{"a":5}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_i64().unwrap(), 5);
}

#[test]
fn parse_float() {
    let s = r#"{"a":5.7}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_f64().unwrap(), 5.7);
}

#[test]
fn parse_bool() {
    let s = r#"{"a":true}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_bool().unwrap(), true);
}

#[test]
fn parse_int_array() {
    let s = r#"{"a":[5, 6, 7]}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"][1].as_i64().unwrap(), 6);
    assert_eq!(doc["a"][2].as_i64().unwrap(), 7);
}

#[test]
fn parse_int_array_newline_as_separator() {
    let s = r#"{"a":[5
    6
    ]}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"][0].as_i64().unwrap(), 5);
    assert_eq!(doc["a"][1].as_i64().unwrap(), 6);
}

#[test]
fn parse_object_newline_as_separator() {
    let s = r#"{"a":5
"b":6
}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_i64().unwrap(), 5);
    assert_eq!(doc["b"].as_i64().unwrap(), 6);
}

#[test]
fn parse_trailing_commas() {
    let s = r#"{"a":[5, 6, 7,
],
}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"][1].as_i64().unwrap(), 6);
}

#[test]
fn parse_nested() {
    let s = r#"{"a":{"b":[{"c":5},{"c":6}]}}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"]["b"][1]["c"].as_i64().unwrap(), 6);
}

#[test]
fn parse_newlines() {
    let s = r#"{"a":
    5
    }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_i64().unwrap(), 5);
}

#[test]
fn parse_comment() {
    let s = r#"{
    // comment 0
    "a":5, // comment1
    "b": 6 # comment 2
    # comment 3
    "c": [7 // comment 4
    # comment 5

    // comment 6
    8]
}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_i64().unwrap(), 5);
}

#[test]
fn parse_keyvalue_separator() {
    let s = r#"{"a":5,"b"=6,"c" {"a":1}}}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_i64().unwrap(), 5);
    assert_eq!(doc["b"].as_i64().unwrap(), 6);
    assert_eq!(doc["c"]["a"].as_i64().unwrap(), 1);
}

#[test]
fn parse_object_merging() {
    let s = r#"{
            "foo" : { "a" : 42 },
            "foo" : { "b" : 43 }
        }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"]["a"].as_i64().unwrap(), 42);
    assert_eq!(doc["foo"]["b"].as_i64().unwrap(), 43);
}

#[test]
fn parse_change_type_to_object() {
    let s = r#"{
            "foo" : [0, 1, 2],
            "foo" : { "b" : 43 }
        }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert!(doc["foo"][0].as_i64().is_none());
    assert_eq!(doc["foo"]["b"].as_i64().unwrap(), 43);
}

#[test]
fn parse_change_type_to_array() {
    let s = r#"{
            "foo" : { "b" : 43 },
            "foo" : [0, 1, 2],
        }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"][0].as_i64().unwrap(), 0);
    assert!(doc["foo"]["b"].as_i64().is_none());
}

#[test]
fn parse_reset_array_index() {
    let s = r#"{
            "foo" : [0, 1, 2],
            "foo" : [5, 6, 7]
        }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"][0].as_i64().unwrap(), 5);
}

#[test]
fn parse_error() {
    let s = r#"{
            "foo" : { "a" : 42 },
            "foo" : {
        }"#;
    let doc = dbg!(HoconLoader::new().load_str(dbg!(s)));

    assert!(doc.is_err());
}

#[test]
fn wrong_index() {
    let s = r#"{ "a" : 42 }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    if let Hocon::BadValue = doc["missing"] {

    } else {
        assert!(false);
    }
    if let Hocon::BadValue = doc[0] {

    } else {
        assert!(false);
    }
}

#[test]
fn wrong_casts() {
    let s = r#"{ "a" : 42 }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert!(doc["missing"].as_i64().is_none());
    assert!(doc["missing"].as_f64().is_none());
    assert!(doc["missing"].as_bool().is_none());
    assert!(doc["missing"].as_string().is_none());
}

#[test]
fn parse_root_braces_omitted() {
    let s = r#""foo" : { "b" : 43 }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"]["b"].as_i64().unwrap(), 43);
}

#[test]
fn parse_unquoted_string() {
    let s = r#"{"foo" : { b : hello world }}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"]["b"].as_string().unwrap(), "hello world");
}

#[test]
fn parse_path() {
    let s = r#"{foo.b : hello }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"]["b"].as_string().unwrap(), "hello");
}

#[test]
fn parse_concat() {
    let s = r#"{"foo" : "hello"" world n°"1 }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"].as_string().unwrap(), "hello world n°1");
}

#[test]
fn parse_path_substitution() {
    let s = r#"{"who" : "world", "number": 1, "bar": "hello "${who}" n°"${number} }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["bar"].as_string().unwrap(), "hello world n°1");
}

#[test]
fn parse_file_ends_with_unquoted_string() {
    let s = r#"#
foo:bar"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["foo"].as_string().unwrap(), "bar");
}

#[test]
fn parse_comment_in_array_no_comma() {
    let s = r#"a=[1 // zut
        2]"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"][0].as_i64().unwrap(), 1);
}

#[test]
fn parse_substitute_array() {
    let s = r#"a=[1, 2, 3],b=[${a}]"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["b"][0][1].as_i64().unwrap(), 2);
}

#[test]
fn parse_empty_objects() {
    let s = r#"a={b{}},b=5"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["b"].as_i64().unwrap(), 5);
}

#[test]
fn parse_missing_substitution() {
    let s = r#"{a={c=${?b}}}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"]["c"], Hocon::BadValue);
}

#[test]
fn parse_empty_object() {
    let s = r#"a=[{},{}],b=[]"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"][0], Hocon::Hash(HashMap::new()));
    assert_eq!(doc["b"], Hocon::Array(vec![]));
}

#[test]
fn parse_comment_after_object() {
    let s = r#"{
    a = {
        b = 2
    }
    # zut
}
#zut"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"]["b"].as_i64().unwrap(), 2);
}

#[test]
fn substitute_before_and_after() {
    let s = r#"{"a" : "before", "before": ${a}, "after": ${b}, "b": "after" }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["before"].as_string().unwrap(), "before");
    assert_eq!(doc["after"].as_string().unwrap(), "after");
}

#[test]
fn environment_variable() {
    std::env::set_var("MY_VAR_TO_TEST", "GREAT_VALUE");

    let s = r#"{"var" : ${MY_VAR_TO_TEST} }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["var"].as_string().unwrap(), "GREAT_VALUE");
}

#[test]
fn environment_variable_disabled() {
    std::env::set_var("MY_VAR_TO_TEST", "GREAT_VALUE");

    let s = r#"{"var" : ${MY_VAR_TO_TEST} }"#;
    let doc: Hocon = dbg!(HoconLoader::new().no_system().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["var"], Hocon::BadValue);
}

#[test]
fn parse_triple_quote() {
    let s = r#"{"a" : """my single line string""" }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_string().unwrap(), "my single line string");
}

#[test]
fn parse_multiline_string() {
    let s = r#"{"a" : """my
multi
line
string""" }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(
        doc["a"].as_string().unwrap(),
        r#"my
multi
line
string"#
    );
}

#[test]
fn parse_triple_quote_with_extra_quote() {
    let s = r#"{"a" : """foo"""" }"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_string().unwrap(), r#"foo""#);
}

#[test]
fn parse_multiple_triple_quote_strings() {
    let s = r#"{"a" : """foo"""", b: """hohoho""", c: """my
multi
line
string"""""}"#;
    let doc: Hocon = dbg!(HoconLoader::new().load_str(dbg!(s)))
        .unwrap()
        .hocon()
        .unwrap();

    assert_eq!(doc["a"].as_string().unwrap(), r#"foo""#);
    assert_eq!(doc["b"].as_string().unwrap(), r#"hohoho"#);
    assert_eq!(
        doc["c"].as_string().unwrap(),
        r#"my
multi
line
string"""#
    );
}
