# Rust Ping Server

## Description
Un serveur HTTP minimaliste en Rust qui :
- Retourne les headers d’une requête GET `/ping` au format JSON.
- Répond 404 sans contenu pour toute autre requête.
- Écoute sur un port configurable via `PING_LISTEN_PORT` (défaut `8080`).

## Installation
1. Installer Rust :  
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Cloner ce dépôt et compiler :  
   ```sh
   cargo build --release
   ```

## Utilisation
### Démarrer le serveur :
```sh
PING_LISTEN_PORT=8081 ./target/release/rust-ping-server
```

### Tester :
```sh
curl -i http://localhost:8081/ping
```

### Exemple de réponse :
```json
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 78

{"headers": ["Host: localhost:3000", "User-Agent: curl/8.9.1", "Accept: */*"]}
```

### Requête invalide :
```sh
curl -i http://localhost:8081/toto
curl -i -X POST http://localhost:8081/ping
```
Renvoie :
```http
HTTP/1.1 404 Not Found
Content-Length: 0
```