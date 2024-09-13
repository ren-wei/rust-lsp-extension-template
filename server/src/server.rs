use std::sync::Arc;

use core::fmt::Debug;
use lsp_textdocument::TextDocuments;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover,
    HoverParams, InitializeParams, InitializeResult, InitializedParams, ServerCapabilities,
    ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp::{Client, LanguageServer};
use tracing::{error, info, instrument, warn};

pub struct LspServer {
    _client: Client,
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
            _client: client,
            is_shared,
            text_documents,
        }
    }
}

impl Debug for LspServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LspServer")
            .field("_client", &self._client)
            .field("is_shared", &self.is_shared)
            .finish()
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

    #[instrument]
    async fn initialized(&self, _params: InitializedParams) {
        info!("start");
        info!("done");
    }

    #[instrument]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        info!("start");
        if !self.is_shared {
            let mut text_documents = self.text_documents.write().await;
            text_documents.listen(
                "textDocument/didOpen",
                &serde_json::to_value(&params).unwrap(),
            );
        }
        info!("done");
    }

    #[instrument]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        info!("start");
        if !self.is_shared {
            let mut text_documents = self.text_documents.write().await;
            text_documents.listen(
                "textDocument/didChange",
                &serde_json::to_value(&params).unwrap(),
            );
        }
        info!("done");
    }

    #[instrument]
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("start");
        if !self.is_shared {
            let mut text_documents = self.text_documents.write().await;
            text_documents.listen(
                "textDocument/didClose",
                &serde_json::to_value(&params).unwrap(),
            );
        }
        info!("done");
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        error!("method not found");
        Err(Error::method_not_found())
    }

    async fn shutdown(&self) -> Result<()> {
        warn!("shutdown");
        Ok(())
    }
}
