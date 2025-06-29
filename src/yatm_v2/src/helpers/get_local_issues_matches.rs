use crate::types::LocalIssue;
use octocrab::models::issues::Issue as GithubIssue;

/// The types of matches between a local issue and a github issue
/// Canonical Match is matching permutation and shortname
#[derive(Eq, PartialEq)]
pub enum IssueMatchType {
    Missing,         // No equivalant GitHub Issue found
    Match,           // Matching github issue found
    MatchedWithDiff, // Matching GitHub Issue found but with some differences
}

pub struct GithubIssueMatches {
    pub local_issue: LocalIssue,
    pub github_issue: Option<GithubIssue>,
    pub match_type: IssueMatchType,
}

/// Get local issues that match upstream
pub fn get_local_issues_matches(
    local_issues: &Vec<LocalIssue>,
    github_issues: &Vec<GithubIssue>,
) -> Vec<GithubIssueMatches> {
    let mut results: Vec<GithubIssueMatches> = Vec::<GithubIssueMatches>::new();
    for local_issue in local_issues {
        let gh_issue = github_issues
            .iter()
            .find(|i| is_local_issue_match_github_issue(local_issue, i));

        if gh_issue.is_some() {
            results.push(GithubIssueMatches {
                local_issue: local_issue.clone(),
                github_issue: gh_issue.clone().cloned(),
                match_type: if is_local_issue_identical_github_issue(local_issue, gh_issue.unwrap())
                {
                    IssueMatchType::Match
                } else {
                    IssueMatchType::MatchedWithDiff
                },
            });
        } else {
            results.push(GithubIssueMatches {
                local_issue: local_issue.clone(),
                github_issue: None,
                match_type: IssueMatchType::Missing,
            });
        }
    }
    results
}

// TODO(tfoote) Change this to a specific tag match
fn is_local_issue_match_github_issue(local_issue: &LocalIssue, github_issue: &GithubIssue) -> bool {
    for label in local_issue.labels.iter() {
        // let github_issue_label_names = github_issue.labels.iter();
        if !github_issue
            .labels
            .iter()
            .any(|github_label| &github_label.name == label)
        {
            return false;
        }
    }
    return true;
}

fn is_local_issue_identical_github_issue(
    local_issue: &LocalIssue,
    github_issue: &GithubIssue,
) -> bool {
    let title_match: bool = if local_issue.title == github_issue.title {
        true
    } else {
        false
    };

    let body_match: bool =
        if local_issue.text_body == github_issue.body.clone().unwrap_or("".to_string()) {
            true
        } else {
            false
        };

    //TODO (tfoote) check for missing labels

    // Full match
    title_match && body_match
}

#[cfg(test)]
mod test_get_local_issues_matches {
    use crate::helpers::get_local_issues_matches;
    use crate::types::LocalIssue;
    use octocrab::models::issues::Issue as GithubIssue;

    // use super::get_local_issues_matches;
    use super::{IssueMatchType, GithubIssueMatches};

    #[test]
    fn matches() {
        let local_issues = vec![
            LocalIssue {
                labels: vec!["label1".to_string()],
                title: "title".to_string(),
                text_body: "text_body".to_string(),
            },
            LocalIssue {
                labels: vec!["label2".to_string()],
                title: "title2".to_string(),
                text_body: "text_body2".to_string(),
            },
            LocalIssue {
                labels: vec!["label3".to_string()],
                title: "identical".to_string(),
                text_body: "text_body3".to_string(),
            },
        ];
        let github_issues: Vec<_> = vec![
            GithubIssue {
                labels: vec![String::from("label3")],
                title: "identical".to_string(),
                body: Some("text_body3".to_string()),
            },
            GithubIssue {
                labels: vec!["label4".to_string()],
                title: "Extra".to_string(),
                body: Some("".to_string()),
            },
        ];
        let result = get_local_issues_matches(&local_issues, &github_issues);

        let missing_count: i32 = 0;
        let match_count: i32 = 0;
        let match_diff_count: i32 = 0;


        for imatch in result.iter() {
            match imatch.match_type {
                IssueMatchType::Match => match_count += 1,
                IssueMatchType::MatchedWithDiff => match_diff_count +=1,
                IssueMatchType::Missing => missing_count +=1,
            }
        }
        assert_eq!(missing_count, 1);
        assert_eq!(match_count, 1);
        assert_eq!(match_diff_count, 1);
    }
}
//     #[test]
//     fn no_matches() {
//         let local_issues = vec![
//             LocalIssue {
//                 labels: vec!["label1".to_string()],
//                 title: "title".to_string(),
//                 text_body: "text_body".to_string(),
//             },
//             LocalIssue {
//                 labels: vec!["label2".to_string()],
//                 title: "title2".to_string(),
//                 text_body: "text_body2".to_string(),
//             },
//         ];
//         let github_issues: Vec<_> = vec![
//             GithubIssueHelper {
//                 labels: vec!["label1".to_string()],
//                 title: "title".to_string(),
//             },
//             GithubIssueHelper {
//                 labels: vec!["label2".to_string()],
//                 title: "title2".to_string(),
//             },
//         ];
//         let result = get_local_issues_without_matches_helper(&local_issues, &github_issues);
//         assert_eq!(result.len(), 0);
//     }

//     #[test]
//     fn one_match() {
//         let local_issues = vec![
//             LocalIssue {
//                 labels: vec!["label1".to_string()],
//                 title: "title".to_string(),
//                 text_body: "text_body".to_string(),
//             },
//             LocalIssue {
//                 labels: vec!["label2".to_string()],
//                 title: "title2".to_string(),
//                 text_body: "text_body2".to_string(),
//             },
//         ];
//         let github_issues: Vec<_> = vec![
//             GithubIssueHelper {
//                 labels: vec!["label1".to_string()],
//                 title: "title".to_string(),
//             },
//             GithubIssueHelper {
//                 labels: vec!["label3".to_string()],
//                 title: "title3".to_string(),
//             },
//         ];
//         let result = get_local_issues_without_matches_helper(&local_issues, &github_issues);
//         assert_eq!(result.len(), 1);
//         assert_eq!(result[0].title, "title2");
//     }
// }

// fn is_local_issue_match_github_issue_helper(
//     local_issue: &LocalIssue,
//     github_issue: &GithubIssueHelper,
// ) -> bool {
//     if local_issue.title != github_issue.title {
//         return false;
//     }
//     if local_issue.labels.len() > github_issue.labels.len() {
//         return false;
//     }
//     for label in local_issue.labels.iter() {
//         if !github_issue
//             .labels
//             .iter()
//             .any(|github_label| github_label == label)
//         {
//             return false;
//         }
//     }
//     return true;
// }

// #[cfg(test)]
// mod test_is_local_issue_match_github_issue {
//     use super::is_local_issue_match_github_issue_helper;
//     use super::{GithubIssueHelper, LocalIssue};

//     #[test]
//     fn is_match() {
//         let local_issue = LocalIssue {
//             labels: vec!["label1".to_string()],
//             title: "title".to_string(),
//             text_body: "text_body".to_string(),
//         };
//         let github_issue = GithubIssueHelper {
//             labels: vec!["label1".to_string()],
//             title: "title".to_string(),
//         };
//         assert!(is_local_issue_match_github_issue_helper(
//             &local_issue,
//             &github_issue
//         ));
//     }

//     #[test]
//     fn is_match_with_github_issue_having_more_labels() {
//         let local_issue = LocalIssue {
//             labels: vec!["label1".to_string()],
//             title: "title".to_string(),
//             text_body: "text_body".to_string(),
//         };
//         let github_issue = GithubIssueHelper {
//             labels: vec!["label1".to_string(), "label2".to_string()],
//             title: "title".to_string(),
//         };
//         assert!(is_local_issue_match_github_issue_helper(
//             &local_issue,
//             &github_issue
//         ));
//     }

//     #[test]
//     fn is_not_match_label() {
//         let local_issue = LocalIssue {
//             labels: vec!["label1".to_string()],
//             title: "title".to_string(),
//             text_body: "text_body".to_string(),
//         };
//         let github_issue = GithubIssueHelper {
//             labels: vec!["label2".to_string()],
//             title: "title".to_string(),
//         };
//         assert!(!is_local_issue_match_github_issue_helper(
//             &local_issue,
//             &github_issue
//         ));
//     }

//     #[test]
//     fn is_not_match_title() {
//         let local_issue = LocalIssue {
//             labels: vec!["label1".to_string()],
//             title: "title".to_string(),
//             text_body: "text_body".to_string(),
//         };
//         let github_issue = GithubIssueHelper {
//             labels: vec!["label1".to_string()],
//             title: "title2".to_string(),
//         };
//         assert!(!is_local_issue_match_github_issue_helper(
//             &local_issue,
//             &github_issue
//         ));
//     }

//     #[test]
//     fn is_not_match_label_and_title() {
//         let local_issue = LocalIssue {
//             labels: vec!["label1".to_string()],
//             title: "title".to_string(),
//             text_body: "text_body".to_string(),
//         };
//         let github_issue = GithubIssueHelper {
//             labels: vec!["label2".to_string()],
//             title: "title2".to_string(),
//         };
//         assert!(!is_local_issue_match_github_issue_helper(
//             &local_issue,
//             &github_issue
//         ));
//     }

//     #[test]
//     fn is_not_match_missing_label() {
//         let local_issue = LocalIssue {
//             labels: vec!["label1".to_string(), "label2".to_string()],
//             title: "title".to_string(),
//             text_body: "text_body".to_string(),
//         };
//         let github_issue = GithubIssueHelper {
//             labels: vec!["label2".to_string()],
//             title: "title".to_string(),
//         };
//         assert!(!is_local_issue_match_github_issue_helper(
//             &local_issue,
//             &github_issue
//         ));
//     }
// }
