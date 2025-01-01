# Derailed Server

Welcome! This is the source code and primary codebase for Ekranoplan, X15, and other core backend services.

## Architecture

### Summary ✨

Derailed is split up between a Rust codebase and an Elixir codebase,
primarily belonging to the API and Gateway respectively.

While these codebases may share some code, they primarily communicate via
gRPC and so only are really directly connected with it.

### Ekranoplan

Ekranoplan is an Axum application, and serves all non-real-time non-websocket data to users as well as other servers
in the future.

### X15

X15 is an Erlang/Elixir application in charge of serving all real-time data, from notifications to messages and
Voice Channel speaking updates, from backend services to users.
