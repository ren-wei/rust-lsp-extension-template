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
        path.join("server", "target", ...(process.platform === "darwin" ? ["release", "rust-lsp-extension-template-server"] : process.platform === "win32" ? ["x86_64-pc-windows-gnu", "release", "rust-lsp-extension-template-server.exe"] : ["x86_64-unknown-linux-musl", "release", "rust-lsp-extension-template-server"]))
    );

    const serverOptions: ServerOptions = {
        run: { command: serverModule },
        debug: {
            command: "cargo",
            args: ["run"],
            options: {
                cwd: context.asAbsolutePath("server"),
            },
        },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "typescript" }],
    };

    // Create the language client and start the client.
    client = new LanguageClient(
        "rust-lsp-extension-template",
        "rust-lsp-extension-template",
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
