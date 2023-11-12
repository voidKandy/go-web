package routes

import (
	"fmt"
	"html/template"
	"net/http"
	"strings"
)

type PageVariables struct {
	Path   string
	Params string
}

func Middleware(h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("Hit middleware from", r.URL)
		hxReq := r.Header.Get("Hx-Request")
		if hxReq == "" {
			// Non-Hx-Request detected, extract path and params from the request URL
			urlParts := strings.Split(r.URL.String(), "?")
			fmt.Println("Url Parts: ", urlParts)
			path := urlParts[0]
			params := ""
			if len(urlParts) > 1 {
				params = string('?') + urlParts[1]
			}

			// Create PageVariables with extracted path and params
			pageVariables := PageVariables{
				Path:   path,
				Params: params,
			}

			// Read the contents of the template file
			tmpl := template.Must(template.ParseFiles("html/index.html", "html/landing.html"))

			// Execute the template with the variables
			err := tmpl.Execute(w, pageVariables)
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
			fmt.Println("template executed")
			return
		}

		// Hx-Request detected, continue with the original handler
		fmt.Println("Rx is Hx, continuing")
		h.ServeHTTP(w, r)
	})
}
