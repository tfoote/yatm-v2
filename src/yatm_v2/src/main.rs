mod utils;

use crate::utils::template::get_github_issue_content;

use common::types::{Action, Expect, Step, TestCase};
use std::collections::HashMap;

fn main() {
    let mut selected_permutation: HashMap<String, String> = HashMap::new();
    selected_permutation.insert("key".to_string(), "value".to_string());
    selected_permutation.insert("key2".to_string(), "value2".to_string());

    let test_case = TestCase {
        requirement: common::types::Requirement {
            name: "requirement".to_string(),
            description: "description".to_string(),
            labels: Some(vec!["req-label".to_string()]),
            links: None,
            steps: vec![
                common::types::Step {
                    action: vec![
                        common::types::Action::StdIn(common::types::Terminal {
                            number: 1,
                            text: "text".to_string(),
                        }),
                        common::types::Action::Image("image".to_string()),
                        common::types::Action::Describe("describe".to_string()),
                    ],
                    expect: vec![common::types::Expect::StdOut(common::types::Terminal {
                        number: 1,
                        text: "text".to_string(),
                    })],
                },
                common::types::Step {
                    action: vec![common::types::Action::StdIn(common::types::Terminal {
                        number: 1,
                        text: "text 2".to_string(),
                    })],
                    expect: vec![common::types::Expect::StdOut(common::types::Terminal {
                        number: 1,
                        text: "text 2".to_string(),
                    })],
                },
            ],
        },
        builder_used: common::types::TestCasesBuilder {
            name: "builder".to_string(),
            description: "description".to_string(),
            labels: Some(vec!["builder-label".to_string()]),
            set: vec![common::types::SetSteps::Include(common::types::Filter {
                all_labels: Some(vec!["label".to_string()]),
                any_names: Some(vec!["name".to_string()]),
                negate: false,
            })],
            permutations: std::collections::HashMap::new(),
            version: 1,
        },
        // selected_permutation: std::collections::HashMap::new(),
        selected_permutation,
    };

    println!("{:?}", get_github_issue_content(test_case).unwrap().labels);
}
