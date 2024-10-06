use crate::models::rule_model::RuleModel;

#[derive(Clone)]
pub struct TokenModel {
    pub token : String,
    pub rule : Option<RuleModel>,
    pub childrens: Vec<Box<TokenModel>>,
}

impl TokenModel {
    pub fn new( token: String, rule: Option<RuleModel>, childrens: Vec<Box<TokenModel>> ) -> Self {
        TokenModel { token, rule, childrens }
    }
}