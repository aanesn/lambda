package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "hello, world!")
	})

	addr := ":8080"
	fmt.Printf("listening on http://localhost%s\n", addr)
	http.ListenAndServe(addr, nil)
}
