package main

import (
	"fmt"
	"log"
	"net/http"
	"voidkandy-dot-dev/routes"
	"voidkandy-dot-dev/routes/data"
)

func main() {
	routes.InitializePageRoutes()
	data.InitializeDataRoutes()
	fmt.Println("Listening on port 3000")
	log.Fatal(http.ListenAndServe(":3000", nil))
}
