use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LeetcodeRequest {
    pub query: String,
    pub variables: LeetcodeVariables
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeetcodeVariables {
    pub user_slug: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LeetcodeStats {
    easy_solved: i64,
    medium_solved: i64,
    hard_solved: i64
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StatsVo {
    status: String,
    data: Option<LeetcodeStats>
}

impl StatsVo {
    pub fn new(status: String, data: Option<LeetcodeStats>) -> Self {
        StatsVo{
            status,
            data
        }
    }
}

impl LeetcodeStats {
    pub(crate) fn new(easy: i64, medium: i64, hard: i64) -> Self {
        LeetcodeStats {
            easy_solved: easy,
            medium_solved: medium,
            hard_solved: hard
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeetcodeQuestionCountResponse {
    pub data: LeetcodeQuestionCountData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeetcodeQuestionCountData {
    pub user_profile_user_question_progress: UserProfileUserQuestionProgress,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileUserQuestionProgress {
    pub num_accepted_questions: Vec<NumAcceptedQuestion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumAcceptedQuestion {
    pub difficulty: String,
    pub count: i64,
}

