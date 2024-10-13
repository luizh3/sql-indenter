use crate::models::rule_model::{BetweenRuleModel, RuleModel};

#[derive(Clone)]
pub struct TokenModel {
    pub token : String,
    pub rule : Option<RuleModel>,
    pub between_rule: Option<BetweenRuleModel>,
    pub childrens: Vec<Box<TokenModel>>,
}

impl TokenModel {
    pub fn new( token: String, rule: Option<RuleModel>, between_rule: Option<BetweenRuleModel>, childrens: Vec<Box<TokenModel>> ) -> Self {
        TokenModel { token, rule, between_rule, childrens }
    }
}