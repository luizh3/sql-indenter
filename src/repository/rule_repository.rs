use std::path::PathBuf;

use crate::helper::file_helper;
use crate::models::rule_model::RuleModel;

pub fn rules (path : &PathBuf ) -> Vec<RuleModel> {

    let ds_content: String = file_helper::read_file(&path);

    let rules: Vec<RuleModel> = serde_json::from_str( ds_content.as_str() ).expect("Failed to parse JSON");

    rules
}