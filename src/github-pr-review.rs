
use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use github_flows::{
    event_handler, get_octo, listen_to_event,
    octocrab::models::CommentId,
    octocrab::models::webhook_events::{WebhookEvent, WebhookEventPayload},
    octocrab::models::webhook_events::payload::{IssueCommentWebhookEventAction, PullRequestWebhookEventAction},
    GithubLogin,
};
use llmservice_flows::{
    chat::{ChatOptions},
    LLMServiceFlows,
};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    dotenv().ok();
    logger::init();
    log::debug!("Running github-pr-review/main");

    let owner = env::var("github_owner").unwrap_or("juntao".to_string());
    let repo = env::var("github_repo").unwrap_or("test".to_string());

    listen_to_event(&GithubLogin::Default, &owner, &repo, vec!["pull_request", "issue_comment"]).await;
}
