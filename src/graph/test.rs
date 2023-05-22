use super::*;

#[test]
fn caveats_from_json_returns_a_simple_rule() {
    let test_me = serde_json::json!({
        "_and": [
            {
            "team": {
                "members": {
                "directus_users_id": {
                    "id": {
                    "_eq": "$CURRENT_USER"
                    }
                }
                }
            }
            }
        ]
    });
    let result = caveats_from_json(&test_me);
    println!("result {:#?}", result);
    assert_eq!(result, vec!["base:*"])
}

#[test]
fn caveats_from_json_returns_a_complex_rule() {
    let test_me = serde_json::json!({
        "_and": [
        {
            "_and": [
            {
                "members": {
                "_eq": "3423324"
                }
            },
            {
                "members": {
                "directus_users_id": {
                    "id": {
                    "_eq": "$CURRENT_USER"
                    }
                }
                }
            }
            ]
        },
        {
            "_or": [
            {
                "vaults": {
                "_neq": "14233412"
                }
            },
            {
                "members": {
                "_neq": "5612354"
                }
            }
            ]
        },
        {
            "_and": [
            {
                "name": {
                "_ncontains": "243"
                }
            }
            ]
        }
        ]
    });
    let result = caveats_from_json(&test_me);
    println!("result {:#?}", result);
    assert_eq!(result, vec!["base:*"])
}
