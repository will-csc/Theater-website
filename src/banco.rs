use mysql::*;
use mysql::{params, prelude::Queryable};

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

pub fn update_user(nome: String, email: String, senha: String, email_antigo: String) -> Result<(), String> {
    let mut conn = connection().map_err(|e| format!("Erro de conexão: {}", e))?;

    let mut updates = Vec::new();
    let mut values = Vec::new();

    if !nome.trim().is_empty() {
        updates.push("nome_usuario = ?");
        values.push(nome);
    }

    if !email.trim().is_empty() {
        updates.push("email_usuario = ?");
        values.push(email.clone());
    }

    if !senha.trim().is_empty() {
        updates.push("senha_usuario = ?");
        values.push(senha);
    }

    if updates.is_empty() {
        return Err("Nenhum campo preenchido para atualização.".to_string());
    }

    let query = format!(
    "UPDATE usuarios SET {} WHERE email_usuario = ?",
    updates.join(", ")
    );
    values.push(email_antigo);

    // Imprimir a query completa no console com os valores das variáveis
    println!("Consulta SQL: {}", query);
    println!("Valores dos parâmetros: {:?}", values);

    // Converte valores para o tipo aceito pela função exec_drop
    let params = mysql::Params::Positional(values.into_iter().map(Value::from).collect());

    conn.exec_drop(query, params)
        .map_err(|e| format!("Erro ao atualizar usuário: {}", e))?;
    println!("Veio ate aqui");

    Ok(())
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

pub fn delete_user(email_antigo: String) -> Result<(), mysql::Error> {
    let mut conn = connection()?;

    conn.exec_drop(
        "DELETE FROM usuarios WHERE email_usuario = :email",
        params! {
            "email" => email_antigo,
        },
    )?;

    Ok(())
}

