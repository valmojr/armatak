package main

type Mission struct {
	MissionName   string    `json:"missionName"`
	Worldname     string    `json:"worldname"`
	MissionAuthor string    `json:"missionAuthor"`
	MissionType   string    `json:"missionType"`
	Victory       string    `json:"victory"`
	MissionStart  string    `json:"missionStart"`
	MissionEnd    string    `json:"missionEnd"`
	Date          string    `json:"date"`
	ScoreBlue     string    `json:"scoreBlue"`
	ScoreRed      string    `json:"scoreRed"`
	ScoreGreen    string    `json:"scoreGreen"`
	Players       []Player  `json:"players"`
	Kills         []Kill    `json:"kills"`
	FPS           []float64 `json:"fps"`
}

type Player struct {
	UID   string `json:"uid"`
	Name  string `json:"name"`
	Side  string `json:"side"`
	Shots int    `json:"shots"`
	Hits  int    `json:"hits"`
	Squad string `json:"squad"`
	Role  string `json:"role"`
	Class string `json:"class"`
}

type Kill struct {
	Time     string `json:"time"`
	Victim   string `json:"victim"`
	Killer   string `json:"killer"`
	Weapon   string `json:"weapon"`
	Distance string `json:"distance"`
}

type DiscordPayload struct {
	Content  string `json:"content"`
	Username string `json:"username,omitempty"` // Optional field
}
