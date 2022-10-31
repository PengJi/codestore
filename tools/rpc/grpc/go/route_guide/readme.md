# steps
1. compile
```
protoc --go_out=. --go_opt=paths=source_relative \
   --go-grpc_out=. --go-grpc_opt=paths=source_relative \
   routeguide/route_guide.proto
```
2. run server  
`go run server/server.go`
3. run client
`go run client/client.go`
  
# refercens
[grpc-go/examples/route_guide/](https://github.com/grpc/grpc-go/tree/master/examples/route_guide)