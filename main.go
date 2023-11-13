package main

import (
	"fmt"
	"log"
	"net/http"
	"voidkandy-dot-dev/logic/routes"
)

func main() {
	routes.InitializePageRoutes()
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("public/static"))))

	// views.InitializeViewRoutes()
	fmt.Println("Listening on port 3000")
	// fs := http.FileServer(http.Dir("static"))
	log.Fatal(http.ListenAndServe(":3000", nil))
}
