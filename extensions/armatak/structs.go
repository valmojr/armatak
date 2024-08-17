package main

type Payload struct {
	Content  string `json:"content"`
	Username string `json:"username,omitempty"`
}

type Marker struct {
	Longitude float64 `json:"longitude"`
	Latitude  float64 `json:"latitude"`
	Name      string  `json:"name"`
	UID       string  `json:"uid"`
	Type      string  `json:"type,omitempty"`
	Course    int     `json:"course,omitempty"`
	Azimuth   int     `json:"azimuth,omitempty"`
	Speed     int     `json:"speed,omitempty"`
	Battery   int     `json:"battery,omitempty"`
	FOV       int     `json:"fov,omitempty"`
	CE        int     `json:"ce,omitempty"`
	HAE       int     `json:"hae,omitempty"`
	LE        int     `json:"le,omitempty"`
}

type Message struct {
	Message string `json:"uid"`
	Sender  string `json:"sender"`
}
