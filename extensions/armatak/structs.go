package main

type Payload struct {
	Content  string `json:"content"`
	Username string `json:"username,omitempty"`
}

type GeoObject struct {
	UID       string  `json:"uid,omitempty"`
	Longitude float64 `json:"longitude"`
	Latitude  float64 `json:"latitude"`
	Attitude  string  `json:"attitude"`
	Bearing   int     `json:"bearing,omitempty"`
	Distance  int     `json:"distance,omitempty"`
	GeoObject string  `json:"geoObject"`
	How       string  `json:"how,omitempty"`
	Name      string  `json:"name,omitempty"`
	Timeout   int     `json:"timeout,omitempty"`
}

type Message struct {
	Message string `json:"uid"`
	Sender  string `json:"sender"`
}
