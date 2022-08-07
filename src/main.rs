extern crate core;

use crate::model::{
    LeetcodeQuestionCountResponse, LeetcodeRequest, LeetcodeStats, LeetcodeVariables, StatsVo,
};
use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, REFERER};
use reqwest::Error;

use warp::Filter;

mod model;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let get_stats = warp::path::param().and_then(|username: String| async move {
        let result = get_stats_handler(username).await;
        match result {
            Ok(content) => {
                let stats_vo = StatsVo::new("success".to_string(), Some(content));
                Ok(warp::reply::json(&stats_vo))
            },
            Err(_) => Err(warp::reject::reject())
        }
    });
    warp::serve(get_stats).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_stats_handler(username: String) -> Result<LeetcodeStats, Error> {
    let req = new_get_stats_request(&username);
    let client = Client::default();
    get_stats_by_request(&req, client, "https://leetcode.cn").await
}

fn new_get_stats_request(username: &str) -> LeetcodeRequest {
    LeetcodeRequest{
        query: "\n    query userQuestionProgress($userSlug: String!) {\n  userProfileUserQuestionProgress(userSlug: $userSlug) {\n    numAcceptedQuestions {\n      difficulty\n      count\n    }\n}\n}\n    ".to_string(),
        variables: LeetcodeVariables{
            user_slug: username.to_string()
        }
    }
}

async fn get_stats_by_request(
    req: &LeetcodeRequest,
    client: Client,
    base_url: &str,
) -> Result<LeetcodeStats, Error> {
    let resp = client
        .post(format!("{}/graphql/", base_url))
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&req).unwrap())
        .header(
            REFERER,
            format!("{}/{}/", base_url, &req.variables.user_slug),
        )
        .send().await?;
    let text = resp.text().await?;
    let content: LeetcodeQuestionCountResponse =
        serde_json::from_str(&text).unwrap();
    let mut easy = 0;
    let mut medium = 0;
    let mut hard = 0;
    for nums in content
        .data
        .user_profile_user_question_progress
        .num_accepted_questions
    {
        let d = nums.difficulty.as_str();
        match d {
            "EASY" => easy = nums.count,
            "MEDIUM" => medium = nums.count,
            "HARD" => hard = nums.count,
            _ => continue,
        }
    }
    let stats = LeetcodeStats::new(easy, medium, hard);
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use crate::{get_stats_by_request, new_get_stats_request, LeetcodeStats};
    use httpmock::MockServer;
    use reqwest::Client;
    use serde_json::json;

    #[test]
    fn test_serde_request() {
        let username = "linbuxiao";
        let res = new_get_stats_request(username);
        assert_eq!(
            "{\"query\":\"\\n    query userQuestionProgress($userSlug: String!) {\\n  userProfileUserQuestionProgress(userSlug: $userSlug) {\\n    numAcceptedQuestions {\\n      difficulty\\n      count\\n    }\\n}\\n}\\n    \",\"variables\":{\"userSlug\":\"linbuxiao\"}}",
            serde_json::to_string(&res).unwrap());
    }

    #[tokio::test]
    async fn test_api() {
        let server = MockServer::start();
        let mock = server.mock(|when, then|{
            when.method("POST")
                .path("/graphql/");
            then.status(200).json_body(json!({"data":{"userProfileUserQuestionProgress":{"numAcceptedQuestions":[{"difficulty":"EASY","count":120},{"difficulty":"MEDIUM","count":174},{"difficulty":"HARD","count":42}]}}}));
        });
        let username = "linbuxiao";
        let client = Client::default();
        let request = new_get_stats_request(username);
        match get_stats_by_request(&request, client, &server.base_url()).await {
            Ok(stats) => {
                let stats_expected = LeetcodeStats::new(120, 174, 42);
                assert_eq!(stats, stats_expected);
            }
            Err(_) => todo!(),
        };
        mock.assert();
    }

    #[tokio::test]
    async fn test_real_api() {
        let username = "linbuxiao";
        let client = Client::default();
        let request = new_get_stats_request(username);
        let stats = get_stats_by_request(&request, client, "https://leetcode.cn").await.unwrap();
        println!("{:?}", stats);
    }
}
