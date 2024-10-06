
mod models;
mod helper;
mod repository;
mod service;

use std::env;
use models::rule_model::RuleModel;
use crate::models::token_model::TokenModel;

fn main() {

    let args: Vec<String> = env::args().collect();

    let sql : String = String::from("SELECT * FROM alunos WHERE id_aluno = 10");

    println!("Args: {:?}", args);

    let rules : Vec<RuleModel> = service::rule_service::rules();

    let mut tokens : Vec<TokenModel> = service::token_service::to_tokens(&sql);

    let result : String = service::indent_service::process( &rules, &mut tokens );

    // println!("rules: {:#?}", rules);
    //
    // println!("tokens: {:#?}", tokens);

    println!("{:#}", result);

}

