use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Between {
    pub start: Vec<String>,
    pub end: Vec<String>,
    #[serde(default)]
    pub is_concat: bool,
    #[serde(default)]
    pub separator: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleModel {
    pub description: String,
    pub has_tab: bool,
    pub has_break_line: bool,
    pub pattern: String,
    pub words: Vec<String>,
    #[serde(default)]
    pub between: Between,
}