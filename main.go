package main

import (
	"fmt"
	"go-web/handlers"
	"log"
	"net/http"
)

func main() {
	handlers.InitializeRoutes()
	fmt.Println("Listening on port 8000")
	log.Fatal(http.ListenAndServe(":8000", nil))
}
