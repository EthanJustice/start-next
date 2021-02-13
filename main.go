package main

import (
	"html/template"
	"io"
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

// Templates
type Templates struct {
	index  *template.Template
	errors *template.Template
}

func (t *Templates) Render(w io.Writer, name string, data interface{}, cat string) error {
	switch cat {
	case "index":
		return t.index.ExecuteTemplate(w, name, data)
	case "errors":
		return t.errors.ExecuteTemplate(w, name, data)
	default:
		return t.errors.ExecuteTemplate(w, name, data)
	}
}

func Dashboard(w http.ResponseWriter, r *http.Request) {
	t.Render(w, "index.html", "", "index")
}

func PageIndex(w http.ResponseWriter, r *http.Request) {

}

func NewPage(w http.ResponseWriter, r *http.Request) {

}

func GetPage(w http.ResponseWriter, r *http.Request) {

}

func PasteIndex(w http.ResponseWriter, r *http.Request) {

}

func NewPaste(w http.ResponseWriter, r *http.Request) {

}

func GetPaste(w http.ResponseWriter, r *http.Request) {

}

func Settings(w http.ResponseWriter, r *http.Request) {

}

type NotFound struct {
}

func (n NotFound) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	t.Render(w, "404.html", "", "errors")
}

var t = &Templates{
	index:  template.Must(template.ParseFiles("views/layout.html", "views/index.html", "views/layouts/nav.html")),
	errors: template.Must(template.ParseFiles("views/layout.html", "views/errors/404.html", "views/layouts/nav.html")),
}

func main() {
	r := mux.NewRouter()
	r.NotFoundHandler = NotFound{}

	r.HandleFunc("/", Dashboard)

	pages := r.PathPrefix("/pages").Subrouter()
	pages.HandleFunc("/", PageIndex)
	pages.HandleFunc("/new", NewPage)
	pages.HandleFunc("/{page}", GetPage)

	pastes := r.PathPrefix("/paste").Subrouter()
	pastes.HandleFunc("/", PasteIndex)
	pages.HandleFunc("/new", NewPaste)
	pages.HandleFunc("/{paste}", GetPaste)

	r.HandleFunc("/settings", Settings)

	log.Fatal(http.ListenAndServe(":8000", r))
}
