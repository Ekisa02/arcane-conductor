Quick API examples

Run an agent task:

curl -X POST http://127.0.0.1:8080/agent/run -H "Content-Type: application/json" -d '{"goal":"Search for Rust async best practices."}'

Inspect status (replace <id>):

curl http://127.0.0.1:8080/agent/status/<id>
