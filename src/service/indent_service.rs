use std::option::{Option};
use crate::models::rule_model::RuleModel;
use crate::models::token_model::TokenModel;

const EMPTY_STRING: &str = "";

pub fn process(rules: &[RuleModel], tokens: &mut Vec<TokenModel>) -> String {

    apply_rules_to_tokens(rules, tokens);

    to_indented( tokens )
}

fn filter_between_rules(rules: &[RuleModel]) -> Vec<&RuleModel> {
    rules.iter()
        .filter(|rule| !rule.between.start.is_empty() && !rule.between.end.is_empty())
        .collect()
}

fn filter_simple_rules(rules: &[RuleModel]) -> Vec<&RuleModel> {
    rules.iter()
        .filter(|rule| rule.between.start.is_empty() && rule.between.end.is_empty())
        .collect()
}

fn apply_simple_rules( rules : Vec<&RuleModel>, tokens : &mut [TokenModel] ) -> () {
    for token in tokens {
        if let Some(matching_rule) = rules.iter().find(|rule| rule.words.contains(&token.token)) {
            token.rule = Some((*matching_rule).clone());
        }
    }
}

fn apply_concat_rules( token : &TokenModel ) -> String {

    let mut result : String = String::new();

    if let Some( rule ) = &token.childrens[0].rule {

        if(  rule.between.is_concat ) {

            let sentence = token.childrens.iter().fold(String::new(), |mut acc, token| {
                acc.push_str(token.token.as_str() );
                acc.push_str(" ");
                acc
            });

            let mut token_concat = TokenModel::new( sentence, Some( rule.clone() ), Vec::new() );

            result.push_str(&apply_rule(&token_concat));

        } else {

            let sentence = token.childrens.iter().fold(String::new(), |mut acc, token| {
                acc.push_str(&apply_rule(token));
                acc
            });

            result.push_str(&sentence);
        }

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
        result.push_str(&apply_concat_rules( token ));
    }

    result

}

fn apply_between_rules( rules: Vec<&RuleModel>, tokens : &mut Vec<TokenModel> ) -> () {

    let mut rule : Option<&RuleModel> = None;
    let mut between_start_token: Option<&mut TokenModel> = None;

    let mut tokens_iterator = tokens.iter_mut().enumerate();

    let mut remove_indexes: Vec<usize> = Vec::new();

    while let Some( ( index, token )) = tokens_iterator.next() {

        if let Some(matching_rule) = rule {

            if matching_rule.between.end.contains( &token.token ) {
                rule = rules.iter()
                    .find(|rule| rule.between.start.contains(&token.token))
                    .map(|r| *r);

                if rule.is_some() {
                    between_start_token = Some(token);
                } else {
                    between_start_token = None;
                }

                continue;
            }

            if !token.rule.is_none() {
                continue;
            }

            token.rule = Some(matching_rule.clone());

            if let Some(ref mut current_token) = &mut between_start_token {
                current_token.childrens.push( Box::new(token.clone() )) ;
                remove_indexes.push(index);
            }

            continue;
        }

        rule = rules.iter()
            .find(|rule| rule.between.start.contains(&token.token))
            .map(|r| *r);

        between_start_token = Some(token);
    }

    remove_indexes.iter().rev().for_each(|index| {
        tokens.remove(*index);
    })

}

pub fn apply_rules_to_tokens(rules: &[RuleModel], tokens: &mut Vec<TokenModel>) {

    let between_rules = filter_between_rules(rules);
    let simple_rules = filter_simple_rules(rules);

    apply_simple_rules( simple_rules, tokens);
    apply_between_rules( between_rules, tokens );

}