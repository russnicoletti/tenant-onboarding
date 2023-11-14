use std::cell::Ref;
use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json;
use crate::gitsource;


#[derive(Serialize, Deserialize, Clone, Debug)]
struct BypassActors {
    actor_id: i16,
    actor_type: String,
    bypass_mode: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RefName {
    include: [String; 5],
    exclude: [String; 0],
}

/*
impl RefName {
    fn new(include: Vec<String>, exclude: Vec<String>) -> Self {
        Self {
            include: include.to_vec(),
            exclude: exclude.to_vec(),
        }
    }
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Conditions {
    ref_name: RefName
}

trait Rule {
    fn r#type(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DeletionRule {
    r#type: String
}

impl Rule for DeletionRule {
    fn r#type(&self) -> String {
        return self.r#type.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PullRequestParameters {
    dismiss_stale_reviews_on_push: bool,
    require_code_owner_review: bool,
    require_last_push_approval: bool,
    required_approving_review_count: bool,
    required_review_thread_resolution: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PullRequestRule {
    r#type: String,
    parameters: PullRequestParameters
}

impl Rule for PullRequestRule {
    fn r#type(&self) -> String {
        return self.r#type.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RequiredStatusChecksObjects {
    context: String,
    integration_id: i16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RequiredStatusChecksParameters {
    required_status_checks: [RequiredStatusChecksObjects; 2],
    strict_required_status_checks_policy: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RequiredStatusChecksRule {
    r#type: String,
    parameters: RequiredStatusChecksParameters

}

impl Rule for RequiredStatusChecksRule {
    fn r#type(&self) -> String {
        return self.r#type.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct NonFastForwardRule {
    r#type: String,
}


impl Rule for NonFastForwardRule {
    fn r#type(&self) -> String {
        return self.r#type.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RuleSet {
    name: String,
    repo: String,
    target: String,
    enforcement: String,
    bypass_actors: BypassActors,
    conditions: RefName,
    rules: [dyn Rule; 3]
}

#[tokio::main]
pub async fn process_github_branches(config_yaml: &Vec<Yaml> , settings_yaml: &Vec<Yaml>) -> Result<(bool), Box<dyn Error>> {

    let mut created = false;

    let config = &config_yaml[0];
    let tenant_repo = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap();

    let setting = &settings_yaml[0];
    let github_auth_token_result = gitsource::authtoken::get_auth_token(setting);
    if !github_auth_token_result.is_ok() {
        return Result::Err(github_auth_token_result.err().unwrap());
    }
    let github_auth_token = github_auth_token_result.unwrap();

    let github_ruleset_api = format!("https://api.github.com/repos/Fiserv/{}//rulesets", tenant_repo);

    let x = RefName {
        include: [
            format!("{}", "refs/heads/main".to_string()),
            format!("{}", "refs/heads/develop".to_string()),
            format!("{}", "refs/heads/stage".to_string()),
            format!("{}", "refs/heads/preview".to_string()),
            format!("{}", "refs/heads/previous".to_string()),
        ],
        exclude: [],
    };
            /*
    let refName =
        RefName::new(vec![
            "refs/heads/main".to_string(),
            "refs/heads/develop".to_string(),
            "refs/heads/stage".to_string(),
            "refs/heads/preview".to_string(),
            "refs/heads/previous".to_string(),
            ], vec!["".to_string()]);
             */

    Ok(created)
}

fn create_request(method: Method, url: String, github_auth_token: String) -> RequestBuilder {
    let github_client = reqwest::Client::new();
    let req = github_client.request(method, url)
        .bearer_auth(github_auth_token.clone())
        .header("User-Agent", "branch protection")
        .header("Accept", "application/vnd.github+json")
        .timeout(Duration::from_secs(5));

    req
}
