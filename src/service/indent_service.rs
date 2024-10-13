use std::option::{Option};
use crate::models::rule_model::{Between, BetweenRuleModel, RuleModel};
use crate::models::token_model::TokenModel;

const EMPTY_STRING: &str = "";

pub fn process(rules: &[RuleModel], tokens: &mut Vec<TokenModel>) -> String {

    let mut process_tokens = apply_rules_to_tokens(rules, tokens);

    to_indented( &mut process_tokens )
}

fn filter_between_rules(rules: &[RuleModel]) -> Vec<&RuleModel> {
    rules.iter()
        .filter(|rule| !rule.between.is_empty() )
        .collect()
}

fn filter_simple_rules(rules: &[RuleModel]) -> Vec<&RuleModel> {
    rules.iter()
        .filter(|rule| rule.between.is_empty() )
        .collect()
}

fn apply_simple_rules( rules : Vec<&RuleModel>, tokens : &mut [TokenModel] ) -> () {
    for token in tokens {
        if let Some(matching_rule) = rules.iter().find(|rule| rule.words.contains(&token.token)) {
            token.rule = Some((*matching_rule).clone());
        }
    }
}

fn apply_concat_rules( token : &TokenModel, between_rule : &Option<BetweenRuleModel> ) -> String {

    let mut result : String = String::new();

    let mut ds_concat_result : String = String::new();

    for children_token in token.childrens.clone() {

        match between_rule {
            Some( between_rule ) => {

                if( !between_rule.is_concat ) {
                    continue;
                };

                ds_concat_result.push_str(children_token.token.as_str() );
                ds_concat_result.push_str(" ");

                if between_rule.separator == children_token.token {
                    result.push_str(&apply_rule_between(&ds_concat_result,between_rule));
                    ds_concat_result.clear();
                }

            }
            _ => {
                result.push_str(&apply_rule(&children_token));
            }
        }

        if( !children_token.childrens.is_empty() ) {
            ds_concat_result.push_str( apply_concat_rules( &children_token, &children_token.between_rule ).as_str() );
        }

    }

    if !ds_concat_result.is_empty() {
        match between_rule {
            Some( between_rule ) => {
                result.push_str(&apply_rule_between(&ds_concat_result,between_rule));
            }
            _ => {
                result.push_str(&ds_concat_result);
            }
        }
    }

    result
}

fn apply_rule_between( ds_token: &String, between_rule : &BetweenRuleModel ) -> String {

    let mut result : String = String::new();

    if between_rule.has_tab {
        result.push_str("\t");
    }

    result.push_str( &ds_token );

    if( between_rule.has_break_line ) {
        result.push_str("\n");
    }

    result
}

fn apply_rule( token : &TokenModel ) -> String {

    let mut result : String = String::new();

    if let Some( rule ) = &token.rule {

        if rule.has_tab {
            result.push_str("\t");
        }

        result.push_str( &token.token );

        if( rule.has_break_line ) {
            result.push_str("\n");
        }

    }

    result
}

fn to_indented( tokens: &mut Vec<TokenModel> ) -> String {

    let mut result : String = String::new();

    for token in tokens {
        result.push_str(&apply_rule( token ));
        result.push_str(&apply_concat_rules( token, &token.between_rule ));
    }

    result

}

fn is_start_between_token<'a>( rules : &'a Vec<&RuleModel>, ds_token : &String ) -> bool {
    rules.iter().any(|rule|
        rule.between.iter().any(|between_rule| {
            between_rule.start == *ds_token
        })
    )
}

fn process_between_rules<'a>(
    between_rules: &mut Vec<Option<(&'a RuleModel, &'a Between)>>,
    rules: &'a Vec<&RuleModel>,
    between_tokens: &mut Vec<TokenModel>,
    index: usize,
    tokens: &mut Vec<TokenModel>,
    process_tokens: &mut Vec<TokenModel>,
) -> Option<TokenModel> {

    let mut token : TokenModel = tokens[index].clone();

    match between_rules.last() {
        Some(Some((rule, between))) => {

            let is_start_between_token = is_start_between_token( rules, &token.token );

            if between.end != token.token && !is_start_between_token {

                match between_tokens.last_mut() {
                    Some(parent_token) => {
                        let children_token = add_rule_if_none(&token, rule);
                        set_children_rule(&children_token, parent_token);
                    }
                    _ => {}
                }

                return None;

            } else if is_start_between_token {

                token = add_rule_if_none(&token, rule);

            }

            if between.end == token.token {

                between_rules.pop();

                match between_tokens.pop() {
                    Some(token) => {

                        if between_tokens.is_empty() {
                            process_tokens.push(token);
                        } else {
                            set_children_rule( &token,between_tokens.last_mut()? );
                        }

                    },
                    _=> {}
                }

                process_between_rules(
                    between_rules,
                    rules,
                    between_tokens,
                    index,
                    tokens,
                    process_tokens
                );
            }
        }
        _ => {}
    }

    Some(token)
}

fn on_add_token_between<'a>(
    process_token : Option<TokenModel>,
    rules: &'a Vec<&RuleModel>,
    between_rules : &mut Vec<Option<(&'a RuleModel, &'a Between)>>,
    between_tokens : &mut Vec<TokenModel>,
    tokens:  &mut Vec<TokenModel>,
    index: usize,

) -> bool {

    match process_token {
        Some(token) => {

            let mut current_token = token.clone();
            let nested_token = between_rule_by_token( &rules, &token);

            match nested_token {
                Some( ( rule, between ) ) => {
                    current_token.between_rule = between.rule.clone();
                }
                _ => {}
            }

            between_rules.push(nested_token);
            between_tokens.push(current_token);

            false
        }
        _ => {

            let mut current_token = tokens[index].clone();
            let nested_token = between_rule_by_token(&rules, &current_token);

            match nested_token {
                Some( ( rule, between ) ) => {
                    current_token.between_rule = between.rule.clone();
                }
                _ => {}
            }

            if ( nested_token.is_some() ) {
                between_rules.push( nested_token );
                between_tokens.push(current_token);
                return true
            }

            false
        }
    }
}

fn apply_between_rules( rules: Vec<&RuleModel>, tokens : &mut Vec<TokenModel> ) -> Vec<TokenModel> {

    let mut between_rules: Vec<Option<(&RuleModel, &Between)>> = Vec::new();
    let mut between_tokens: Vec<TokenModel> = Vec::new();
    let mut process_tokens : Vec<TokenModel> = Vec::new();;

    for index in 0..tokens.len(){

         let process_token = process_between_rules(
            &mut between_rules,
            &rules,
            &mut between_tokens,
            index,
            tokens,
            &mut process_tokens
        );

         on_add_token_between(
            process_token,
            &rules,
            &mut between_rules,
            &mut between_tokens,
            tokens,
            index,
        );

    }

    match between_tokens.pop() {
        Some(token) => {
            process_tokens.push(token);
        }
        _=> {}
    }

    process_tokens

}

pub fn set_children_rule(child_token: &TokenModel, parent_token: &mut TokenModel ) {
    parent_token.childrens.push(Box::new(child_token.clone()));
}

fn add_rule_if_none(token: &TokenModel, rule: &RuleModel ) -> TokenModel {

    let mut new_token = token.clone();

    if new_token.rule.is_none() {
        new_token.rule = Some(rule.clone());
    }

    new_token
}

fn between_rule_by_token<'a>(  rules: &'a Vec<&RuleModel>, token: &TokenModel ) -> Option<(&'a RuleModel, &'a Between)> {

    for rule in rules {
        if let Some(between) = rule.between.iter().find(|between| between.start == token.token) {
            return Some((rule, between));
        }
    }

    None
}

pub fn apply_rules_to_tokens(rules: &[RuleModel], tokens: &mut Vec<TokenModel>) -> Vec<TokenModel> {

    let between_rules = filter_between_rules(rules);
    let simple_rules = filter_simple_rules(rules);

    apply_simple_rules( simple_rules, tokens);

    apply_between_rules( between_rules, tokens )

}