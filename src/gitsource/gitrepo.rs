use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json::Value;
use reqwest::{Client, Method};
 
#[derive(Serialize, Deserialize, Debug)] 
struct RepoInfo { 
    owner:String,
    name: String,
    description: String,
    private: bool,
    include_all_branches:bool 
}

#[tokio::main]
pub async fn create_repo(config_yaml: &Vec<Yaml> , settings_yaml: &Vec<Yaml>) -> Result<(bool), Box<dyn Error>> {

    let mut created = false;

    let config = &config_yaml[0]; 
    let tenant_repo = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap();

    let setting = &settings_yaml[0]; 
    let github_api = setting["github"]["gitHubAPIUrl"].as_str().unwrap();
    let github_token = setting["github"]["gitHubAuthToken"].as_str().unwrap();
    let github_repo_gen_api = setting["github"]["gitHubTemplateRepo"].as_str().unwrap();    
    let github_owner = setting["github"]["gitHubSourceOwner"].as_str().unwrap();

    println!("Adding new Tenant Repo {:#?}", tenant_repo);
 
 let repo_data = RepoInfo { 
        owner: github_owner.to_string(),
        name: tenant_repo.to_string(),
        description: "A new Tenant repo is generated by DevStudio team".to_string() ,
        private:false,
        include_all_branches:true 
        };
  
    let github_client = reqwest::Client::new();
    let post_req = github_client.request(Method::POST, github_repo_gen_api)
    .bearer_auth(github_token)
    .header("User-Agent", "tenant-onbaording")
    .header("Accept", "application/vnd.github+json")
    .timeout(Duration::from_secs(5))
    .json(&repo_data);

    println!("github_token {}", github_token);


    let resp_data = post_req.send().await?; 
    println!("Adding Repo Status {}", resp_data.status());
    //if (resp_data.status() == reqwest::StatusCode::UNPROCESSABLE_ENTITY) 
    if (resp_data.status() == reqwest::StatusCode::CREATED) 
    {
        let res_body = resp_data.bytes().await?; 
        let str_body = res_body.to_vec();
        let str_response = String::from_utf8_lossy(&str_body);
        println!("Adding Repo Response: {} ", str_response);
        created = true;
    } 
    
    Ok((created))
}

