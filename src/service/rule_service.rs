use std::path::PathBuf;
use crate::helper::file_helper;
use crate::models::rule_model::RuleModel;
use crate::repository::rule_repository;

pub fn rules () -> Vec<RuleModel> {

    let config_path:Vec<String> = file_helper::config_path();

    let path:PathBuf = file_helper::to_file_path(&config_path);

    rule_repository::rules(&path)
}