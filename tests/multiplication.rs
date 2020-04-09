mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;
#[test]
fn ok_multiplication() {
    let data = r#"{"messages":[ {"content":{"text":"8"},"content_type":"text"}],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "start",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// ARRAY
////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiplication_array_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_array_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_array_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_array_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_array_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_array_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_array_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_array_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_array_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_array_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_array_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_array_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_array_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_array_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

////////////////////////////////////////////////////////////////////////////////
/// BOOLEAN
////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiplication_boolean_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_boolean_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_boolean_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_boolean_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_boolean_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_boolean_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_boolean_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_boolean_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_boolean_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_boolean_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_boolean_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_boolean_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_boolean_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_boolean_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

////////////////////////////////////////////////////////////////////////////////
/// FLOAT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiplication_float_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_float_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_float_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_float_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_float_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_float_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn multiplication_float_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_float_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn multiplication_float_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_float_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_float_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_float_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_float_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_float_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

/////////////////////////////////////////////////////////////////////////////////
/// INT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiplication_int_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_int_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_int_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_int_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_int_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_int_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn multiplication_int_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_int_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn multiplication_int_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_int_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_int_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_int_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_int_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_int_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// NULL
////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiplication_null_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_null_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_null_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_null_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_null_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_null_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_null_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_null_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_null_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_null_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_null_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_null_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_null_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_null_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

////////////////////////////////////////////////////////////////////////////////
/// OBJECT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiplication_object_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_object_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_object_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_object_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_object_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_object_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_object_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_object_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_object_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_object_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_object_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_object_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_object_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_object_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

/////////////////////////////////////////////////////////////////////////////////
/// STRING
////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiplication_string_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_string_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_string_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_string_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_string_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_string_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn multiplication_string_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_string_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn multiplication_string_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_string_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_string_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_string_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn multiplication_string_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "multiplication_string_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/multiplication.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
