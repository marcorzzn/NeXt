import http.server
import socketserver
PORT = 8080
class WasmHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()
WasmHandler.extensions_map['.wasm'] = 'application/wasm'
with socketserver.TCPServer(("", PORT), WasmHandler) as httpd:
    print(f"✅ TUTTO PRONTO! Clicca sul pop-up 'Open in Browser' o vai su http://localhost:{PORT}")
    httpd.serve_forever()
