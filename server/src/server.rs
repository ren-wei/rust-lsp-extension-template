use std::fmt::Display;
use std::sync::Arc;

use lsp_textdocument::TextDocuments;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    InitializeParams, InitializeResult, InitializedParams, MessageType, ServerCapabilities,
    ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp::{Client, LanguageServer};

pub struct LspServer {
    client: Client,
    is_shared: bool,
    text_documents: Arc<RwLock<TextDocuments>>,
}

impl LspServer {
    pub fn new(
        client: Client,
        shared_text_documents: Option<Arc<RwLock<TextDocuments>>>,
    ) -> LspServer {
        let is_shared;
        let text_documents = if let Some(shared_text_documents) = shared_text_documents {
            is_shared = true;
            shared_text_documents
        } else {
            is_shared = false;
            Arc::new(RwLock::new(TextDocuments::new()))
        };
        LspServer {
            client,
            is_shared,
            text_documents,
        }
    }

    async fn log<M>(&self, message: M)
    where
        M: Display + Send + Sync,
    {
        self.client.log_message(MessageType::LOG, message).await;
    }

    async fn info<M>(&self, message: M)
    where
        M: Display + Send + Sync,
    {
        self.client.log_message(MessageType::INFO, message).await;
    }

    async fn error<M>(&self, message: M)
    where
        M: Display + Send + Sync,
    {
        self.client.log_message(MessageType::ERROR, message).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LspServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "rust-lsp-extension-template-server".to_string(),
                version: Some("1.0.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.info("initialized").await;
        self.info("initialized done").await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.info("did_open").await;
        if !self.is_shared {
            let mut text_documents = self.text_documents.write().await;
            text_documents.listen(
                "textDocument/didOpen",
                &serde_json::to_value(&params).unwrap(),
            );
        }
        self.info("did_open done").await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.info("did_change").await;
        if !self.is_shared {
            let mut text_documents = self.text_documents.write().await;
            text_documents.listen(
                "textDocument/didChange",
                &serde_json::to_value(&params).unwrap(),
            );
        }
        self.info("did_change done").await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.info("did_close").await;
        if !self.is_shared {
            let mut text_documents = self.text_documents.write().await;
            text_documents.listen(
                "textDocument/didClose",
                &serde_json::to_value(&params).unwrap(),
            );
        }
        self.info("did_close done").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
