module github.com/ekodb/examples

go 1.22

toolchain go1.24.2

replace github.com/ekoDB/ekodb-client-go => ../../../ekodb-client-go

require (
	github.com/ekoDB/ekodb-client-go v0.0.0-00010101000000-000000000000
	github.com/gorilla/websocket v1.5.1
	github.com/joho/godotenv v1.5.1
)

require golang.org/x/net v0.17.0 // indirect
