use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FormData {
    nome: Option<String>,
    email: String,
    senha: String,
}

#[derive(Debug, Deserialize)]
struct EscolhaForm {
    filme: String,
}

// Página inicial (cadastro)
#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("static/cadastro.html").await
}

// Recebe dados do cadastro e redireciona para escolha de filme
#[post("/cadastro")]
async fn cadastro(form: web::Form<FormData>) -> impl Responder {
    println!("Novo cadastro:");
    println!("Nome: {:?}", form.nome);
    println!("Email: {}", form.email);
    println!("Senha: {}", form.senha);

    HttpResponse::Found()
        .append_header(("Location", "/escolher"))
        .finish()
}

// Página de escolha de filme
#[get("/escolher")]
async fn escolher() -> impl Responder {
    NamedFile::open_async("static/escolher.html").await
}

// Recebe filme escolhido e redireciona para pagamento
#[post("/pagamento")]
async fn pagamento(info: web::Form<EscolhaForm>) -> impl Responder {
    println!("Filme escolhido: {}", info.filme);

    // Aqui você poderia guardar essa escolha numa sessão ou banco de dados

    HttpResponse::Found()
        .append_header(("Location", "/pagamento_tela"))
        .finish()
}

// Página de pagamento
#[get("/pagamento_tela")]
async fn pagamento_tela() -> impl Responder {
    NamedFile::open_async("static/pagamento.html").await
}

// Simula o fim do pagamento e libera acesso ao filme
#[post("/finalizar_pagamento")]
async fn finalizar_pagamento() -> impl Responder {
    println!("Pagamento efetuado com sucesso.");

    HttpResponse::Found()
        .append_header(("Location", "/assistir"))
        .finish()
}

// Página para assistir ao filme
#[get("/assistir")]
async fn assistir() -> impl Responder {
    NamedFile::open_async("static/assistir.html").await
}

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
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
