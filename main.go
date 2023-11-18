package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"voidkandy-dot-space/src/routes"
)

func main() {
	routes.InitializePageRoutes()
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("public/static"))))
	port := ":" + os.Getenv("PORT")
	if port == ":" {
		port = ":3000"
	}
	fmt.Println("Listening on port ", port)
	log.Fatal(http.ListenAndServe(port, nil))
}
