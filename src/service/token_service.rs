use crate::models::token_model::TokenModel;

pub fn to_tokens(raw_text : &String ) -> Vec<TokenModel> {
    let mut raw_tokens : Vec<String> = raw_text.split_whitespace().map(|current| current.to_string()).collect();
    raw_tokens.iter().map(|current| TokenModel::new( current.clone(), None, None, Vec::new() )).collect()
}

/* Example */

/*

Case 1:

    Normal:

    SELECT * FROM alunos WHERE id_aluno = 10;

    Expected:

    SELECT
        *
    FROM
        alunos
    WHERE
        id_aluno = 10;

case 2:

    Normal:

    SELECT * FROM alunos a JOIN turmas t ON a.id_aluno = t.id_aluno WHERE id_aluno = 10;

    Expected:

    SELECT
        *
    FROM
        alunos a
        JOIN turmas t ON a.id_aluno = t.id_aluno
    WHERE
        id_aluno = 10;

case 3:

    Normal:

    SELECT id_aluno, ds_aluno FROM alunos a JOIN turmas t ON a.id_aluno = t.id_aluno WHERE id_aluno = 10 AND ds_aluno = 'test';

    Expected:

    SELECT
        id_aluno,
        ds_aluno
    FROM
        alunos a
        JOIN turmas t ON a.id_aluno = t.id_aluno
    WHERE
        id_aluno = 10 AND
        ds_aluno = 'test';

case 4:

    Normal:

    SELECT id_aluno, ds_aluno FROM alunos a JOIN turmas t ON a.id_aluno = t.id_aluno WHERE id_aluno = 10 AND ds_aluno = 'test';

    Expected:

    SELECT
        id_aluno,
        ds_aluno
    FROM
        alunos a
        JOIN turmas t ON a.id_aluno = t.id_aluno
    WHERE
        id_aluno = 10 AND
        ds_aluno = 'test';
 */