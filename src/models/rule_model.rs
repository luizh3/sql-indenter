use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BetweenRuleModel {
    #[serde(default)]
    pub is_concat: bool,
    #[serde(default)]
    pub separator: String,
    #[serde(default)]
    pub has_tab: bool,
    #[serde(default)]
    pub has_break_line: bool,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Between {
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub rule: Option<BetweenRuleModel>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleModel {
    pub description: String,
    pub has_tab: bool,
    pub has_break_line: bool,
    pub pattern: String,
    pub words: Vec<String>,
    #[serde(default)]
    pub between: Vec<Between>,
}