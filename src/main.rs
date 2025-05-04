use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use serde::Deserialize;
mod banco;

#[derive(Debug, Deserialize)]
struct FormData {
    nome: String,
    email: String,
    senha: String,
    confirmar_senha: String
}

#[derive(Debug, Deserialize)]
struct EscolhaForm {
    filme: String,
}

// ---------------------------- Verifica Email --------------------------//
#[get("/verifica_email")]
async fn verifica_email(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    if let Some(email) = query.get("email") {
        match banco::exists_email(email.to_string()) {
            Ok(true) => HttpResponse::Ok().body("existe"),
            Ok(false) => HttpResponse::Ok().body("nao_existe"),
            Err(_) => HttpResponse::InternalServerError().body("erro"),
        }
    } else {
        HttpResponse::BadRequest().body("email_nao_enviado")
    }
}

// ---------------------------- Cadastro --------------------------//
#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("static/cadastro.html").await
}

#[post("/cadastro")]
async fn cadastro(form: web::Form<FormData>) -> impl Responder {

    match banco::insert_user(form.nome.clone(), form.email.clone(), form.senha.clone()) {
        Ok(_) => HttpResponse::Found()
                    .append_header(("Location", "/login"))
                    .finish(),
        Err(e) => {
            eprintln!("Erro ao inserir no banco: {:?}", e);
            HttpResponse::InternalServerError().body("Erro ao cadastrar usuário.")
        }
    }
}

// ---------------------------- Login --------------------------//
#[get("/login")]
async fn login() -> impl Responder {
    NamedFile::open_async("static/login.html").await
}

#[post("/login")]
async fn login_user(form: web::Form<FormData>) -> impl Responder {
    let email = form.email.clone();
    let senha = form.senha.clone();

    match banco::exists_user(email, senha) {
        Ok(true) => HttpResponse::Found()
            .append_header(("Location", "/escolher"))
            .finish(),
        Ok(false) => HttpResponse::Unauthorized().body("Email ou senha inválidos."),
        Err(e) => {
            eprintln!("Erro ao verificar usuário: {:?}", e);
            HttpResponse::InternalServerError().body("Erro ao verificar login.")
        }
    }
}

// ---------------------------- Verifica Login --------------------------//
#[get("/verifica_login")]
async fn verifica_login(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    if let (Some(email), Some(senha)) = (query.get("email"), query.get("senha")) {
        match banco::exists_user(email.clone(), senha.clone()) {
            Ok(true) => HttpResponse::Ok().body("ok"),
            Ok(false) => HttpResponse::Ok().body("invalido"),
            Err(_) => HttpResponse::InternalServerError().body("erro"),
        }
    } else {
        HttpResponse::BadRequest().body("Parâmetros 'email' e 'senha' são obrigatórios.")
    }
}



// ---------------------------- Escolher --------------------------//
#[get("/escolher")]
async fn escolher() -> impl Responder {
    NamedFile::open_async("static/escolher.html").await
}

// ---------------------------- Pagamento --------------------------//
#[post("/pagamento")]
async fn pagamento(info: web::Form<EscolhaForm>) -> impl Responder {
    println!("Filme escolhido: {}", info.filme);

    HttpResponse::Found()
        .append_header(("Location", "/pagamento"))
        .finish()
}

#[get("/pagamento")]
async fn pagamento_tela() -> impl Responder {
    NamedFile::open_async("static/pagamento.html").await
}

#[post("/finalizar_pagamento")]
async fn finalizar_pagamento() -> impl Responder {
    println!("Pagamento efetuado com sucesso.");

    HttpResponse::Found()
        .append_header(("Location", "/assistir"))
        .finish()
}

// ---------------------------- Assistir --------------------------//
#[get("/assistir")]
async fn assistir() -> impl Responder {
    NamedFile::open_async("static/assistir.html").await
}

// ---------------------------- Função Principal --------------------------//
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor rodando em http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(cadastro)
            .service(escolher)
            .service(pagamento)
            .service(pagamento_tela)
            .service(finalizar_pagamento)
            .service(assistir)
            .service(verifica_email)
            .service(verifica_login)
            .service(login)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
