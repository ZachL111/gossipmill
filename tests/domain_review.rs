use gossipmill::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 41, slack: 22, drag: 20, confidence: 92 };
    assert_eq!(review_score(case), 136);
    assert_eq!(review_lane(case), "watch");
}
