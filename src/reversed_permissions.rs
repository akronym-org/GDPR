//! Build a collection of Permission Rules
use crate::dump;
use crate::entities::directus_permissions;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type CollectionRules = HashMap<String, FieldRule>;

/// Represent access permissions for a resource in Directus.
///
/// `ResourceRule` represents all allowed actions on a resource:
/// 'create', 'read', 'update', 'delete', and 'share'.
/// Each action is defined by a unique `ActionRule`.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FieldRule {
    create: Vec<ActionRule>,
    read: Vec<ActionRule>,
    update: Vec<ActionRule>,
    delete: Vec<ActionRule>,
    share: Vec<ActionRule>,
}

/// Represent an action (or verb) on a Directus Resource.
///
/// An `ActionRule` contains roles (UUID), permissions, and
/// validation rules. Both permissions and validation are
/// potentially deeply nested JSON objects, chaining multiple
/// rules with logical AND/OR operators.
///
/// # Properties
/// * `roles` - A vector of rules, potentially deduplicated if the same
/// permissions and validation are valid for multiple roles.
/// * `permissions` - A deeply nested JSON Object reflecting WHO can
/// read a resource or WHICH resources are in scope of that rule.
/// Can potentially contain references to other fields.
/// * `validation` - A deeply nested JSON Object reflecting HOW
/// a resource can look.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ActionRule {
    roles: Vec<String>,
    // TODO: use serde json
    permissions: Value,
    validation: Value,
}

impl FieldRule {
    fn new() -> Self {
        Self {
            create: Vec::new(),
            read: Vec::new(),
            update: Vec::new(),
            delete: Vec::new(),
            share: Vec::new(),
        }
    }

    pub fn iter_keys() -> impl Iterator<Item = &'static str> {
        ["create", "read", "update", "delete", "share"].into_iter()
    }
}

// impl From<dump::Request> for FieldRule {
//     fn from(resource: dump::Request) -> Self {
//         println!("test")
//     }
// }

pub struct Builder {
    requests: Vec<String>,
    context: String,
}

impl Builder {
    fn request(resources: &[dump::Request]) -> Self {
        Self {
            requests: vec![],
            context: "ha".to_owned(),
        }
    }

    /// Organize the database permissions into GDPR's base format.
    ///
    /// # Arguments
    ///
    /// * `args` - A reference to the user's settings used by the dump command.
    /// * `permissions` - A reference to a vector of `directus_permissions::Model` objects
    ///   containing the permissions to be organized.
    pub fn build(
        resources: &[dump::Request],
        input: &Vec<directus_permissions::Model>,
    ) -> CollectionRules {
        let mut dump_all = HashMap::new();
        for field_name in resources {
            let mut field = FieldRule::new();

            // loop over each action
            for key in FieldRule::iter_keys() {
                let role_permission = input
                    .iter()
                    .filter(|permission| permission.action == key)
                    .map(|permission| {
                        let mut map = IndexMap::new();
                        map.insert(
                            "roles".to_owned(),
                            Value::String(
                                permission.role.clone().unwrap_or("public".to_owned()),
                            ),
                        );
                        map.insert(
                            "permissions".to_owned(),
                            permission.permissions.clone().unwrap_or(Value::Null),
                        );
                        map.insert(
                            "validation".to_owned(),
                            permission.validation.clone().unwrap_or(Value::Null),
                        );
                        map
                    })
                    .collect::<Vec<_>>();

                for permission in role_permission {
                    let dump_operation = ActionRule {
                        roles: vec![permission["roles"].as_str().unwrap().to_owned()],
                        permissions: permission["permissions"].clone(),
                        validation: permission["validation"].clone(),
                    };

                    match key {
                        "create" => field.create.push(dump_operation),
                        "read" => field.read.push(dump_operation),
                        "update" => field.update.push(dump_operation),
                        "delete" => field.delete.push(dump_operation),
                        "share" => field.share.push(dump_operation),
                        _ => (),
                    }
                }
            }

            // dump_all.insert(field_name.to_owned(), field);
        }

        return dump_all;
    }
}
