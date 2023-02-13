pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let pr_vec: Vec<&str> = prefix.split("/").filter(|c| !c.is_empty()).collect();
    let rq_vec: Vec<&str> = request_path.split("/").filter(|c| !c.is_empty()).collect();
    if pr_vec.len() > rq_vec.len() {
        return false;
    }

    pr_vec
        .iter()
        .zip(rq_vec.iter().take(pr_vec.len()))
        .all(|p| "*" == *p.0 || *p.0 == *p.1)
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}