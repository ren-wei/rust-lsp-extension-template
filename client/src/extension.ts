import * as path from "path";
import { ExtensionContext } from "vscode";

import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    const serverModule = context.asAbsolutePath(
        path.join("server", "target", ...(process.platform === "darwin" ? ["debug", "rust-lsp-extension-template-server"] : ["x86_64-pc-windows-gnu", "release", "rust-lsp-extension-template-server.exe"]))
    );

    const serverOptions: ServerOptions = {
        run: { command: serverModule },
        debug: { command: serverModule },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "typescript" }, { scheme: "file", language: "vue" }],
    };

    // Create the language client and start the client.
    client = new LanguageClient(
        "locale-service",
        "Locale Service",
        serverOptions,
        clientOptions
    );

    // Start the client. This will also launch the server
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
