
mod models;
mod helper;
mod repository;
mod service;

use std::env;
use std::path::{Path, PathBuf};
use models::rule_model::RuleModel;
use crate::models::token_model::TokenModel;

const NR_ARGS : usize = 2;

fn main() {

    let args: Vec<String> = env::args().collect();

    // let sql : String = String::from("SELECT * FROM alunos WHERE id_aluno = 10 AND id_test = 20");

    println!("Args: {:?}", args);

    if args.len() != NR_ARGS {
        panic!( "Number of arguments needed: {}", args.len() );
    }

    let path_file : String = args[1].clone();

    let path_buf : PathBuf = Path::new(&path_file).to_path_buf();

    let raw_code:String = helper::file_helper::read_file(&path_buf);

    let rules : Vec<RuleModel> = service::rule_service::rules();

    let mut tokens : Vec<TokenModel> = service::token_service::to_tokens(&raw_code);

    let result : String = service::indent_service::process( &rules, &mut tokens );

    helper::file_helper::overwrite_file( &path_buf, &result );

    // println!("rules: {:#?}", rules);

    // println!("tokens: {:#?}", tokens);

    println!("{:#}", result);

}

