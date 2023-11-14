#![cfg(test)]

use crate::github::get_packages_from_github::get_packages;

#[tokio::test]
async fn test_fetch_branch() {
    let testing_branch = "main";
    let fetch_result = get_packages(testing_branch).await.expect("Failed to download package");

    assert_eq!(fetch_result.is_empty(), false);

}