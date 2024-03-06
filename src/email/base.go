package email

import (
	"fmt"
	"html/template"
	"net/http"
)

func SendEmailHandler(w http.ResponseWriter, r *http.Request) {
	type EmailInfo struct {
		IsGet         bool
		IsPostSuccess bool
	}

	tmpl := template.Must(template.ParseFiles("public/html/partials/email-form.html"))
	if r.Method == http.MethodGet {
		info := EmailInfo{
			IsGet:         true,
			IsPostSuccess: false,
		}
		err := tmpl.Execute(w, info)
		if err != nil {
			fmt.Println(err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
		return
	}

	if r.Method == http.MethodPost {

		r.ParseForm()
		web3form_url := "https://api.web3forms.com/submit"
		r.Form.Set("access_key", "b896a032-13cb-4639-a9ad-1fc1aacb1255")
		res, emailErr := http.PostForm(web3form_url, r.Form)
		fmt.Println("WEB3 RESPONSE: ", res)

		info := EmailInfo{
			IsGet:         false,
			IsPostSuccess: res.StatusCode == 200,
		}

		e := tmpl.Execute(w, info)
		if e != nil {
			fmt.Println(emailErr)
			http.Error(w, emailErr.Error(), http.StatusInternalServerError)
		}
		return
	}

	http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
}
