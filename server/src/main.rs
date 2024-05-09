use std::sync::Arc;

use rust_lsp_extension_template_server::{log::LspSubscriber, server::LspServer};
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| {
        let client = Arc::new(client);
        let subscriber = LspSubscriber::new(Arc::clone(&client));
        tracing::subscriber::set_global_default(subscriber).unwrap();
        LspServer::new(client, None)
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
