package main

type Payload struct {
	Content  string `json:"content"`
	Username string `json:"username,omitempty"` // Optional field
}
