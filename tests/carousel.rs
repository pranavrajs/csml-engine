mod support;

use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_card() {
    let data = r#" {
        "messages":[
            {
                "content": {
                    "subtitle": "c1",
                    "image_url": "url",
                    "buttons": [
                        {
                                "accepts": ["b1"],
                                "button_type": "quick_button",
                                "payload": "b1",
                                "title": "b1",
                                "theme": "primary",
                            
                                "content": {"payload": "b1", "title": "b1"},
                                "content_type": "button"
                        }
                    ]
                },
                "content_type": "card"
            }
        ],
        "memories": []
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/carousel.csml",
        "simple_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_carousel() {
    let data = r#"
    {"messages":
        [ {
            "content": {
                "cards": [
                    {
                        "subtitle": "c1",
                        "buttons": [
                            {
                                    "accepts": ["b1"],
                                    "button_type": "quick_button",
                                    "payload": "b1",
                                    "title": "b1",
                                    "theme": "primary",
                                
                                    "content": {"payload": "b1", "title": "b1"},
                                    "content_type": "button"
                            }
                        ]
                    }
                ]
            },
            "content_type": "carousel"
        } ],
    "memories":[]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/carousel.csml",
        "start",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
#[test]
fn ok_carousel_step1() {
    let data = r#"
    {"messages":
        [ {
            "content": {
                "cards": [
                    {
                        "subtitle": "c1",
                        "buttons": [
                            {
                                    "accepts": ["b1"],
                                    "button_type": "quick_button",
                                    "payload": "b1",
                                    "title": "b1",
                                    "theme": "primary",
                                    "icon": "info",

                                    "content": {"payload": "b1", "title": "b1"},
                                    "content_type": "button"
                            }
                        ]
                    }
                ]
            },
            "content_type": "carousel"
        } ],
    "memories":[]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/carousel.csml",
        "carousel1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
