use mysql::*;
use mysql::prelude::*;

fn connection() -> Result<PooledConn, mysql::Error>{
    let url = "mysql://root:Sny-301mv@localhost:3306/cine";
    let pool = Pool::new(url)?;
    pool.get_conn()
}

pub fn insert_user(nome: String,email: String,senha: String) -> Result<(), mysql::Error>{
    let mut conn: PooledConn = connection()?;

    conn.exec_drop(
        "INSERT INTO usuarios (nome_usuario,email_usuario,senha_usuario) 
         VALUES (:nome_usuario, :email_usuario, :senha_usuario)",
        params! {
            "nome_usuario" => nome,
            "email_usuario" => email,
            "senha_usuario" => senha
        }
    )
}

pub fn exists_email(email: String) -> Result<bool, mysql::Error> {
    let mut conn = connection()?; // Conexão com o banco

    // Especificamos que esperamos uma tupla com um único campo String
    match conn.exec_first::<(String,), _, _>(
        "SELECT email_usuario FROM usuarios WHERE email_usuario = :email",
        params! {
            "email" => email,
        },
    ) {
        Ok(result) => Ok(result.is_some()),
        Err(e) => {
            eprintln!("Erro ao executar a consulta: {:?}", e); // Log de erro
            Err(e)
        }
    }
}

pub fn exists_user(email: String, password: String) -> Result<bool,mysql::Error>{
    let mut conn = connection()?; // Pode falhar aqui


    let result: Option<(String,)> = conn.exec_first(
        "SELECT senha_usuario FROM usuarios WHERE email_usuario = :email",
        params! {
            "email" => email
        },
    )?;

    match result {
        Some((stored_password,)) => {
            Ok(stored_password == password)
        }
        None => {
            Ok(false)
        }
    }
}

