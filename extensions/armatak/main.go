package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"path"

	"github.com/indig0fox/a3go/a3interface"
	"github.com/indig0fox/a3go/assemblyfinder"
)

// modulePath is the absolute path to the compiled DLL, which should be the addon folder
var modulePath string = assemblyfinder.GetModulePath()

// modulePathDir is the containing folder
var modulePathDir string = path.Dir(modulePath)

var EXTENSION_NAME = "STATS_LOGGER"

var mission Mission

var RVExtensionChannels = map[string]chan string{
	":timeNow:": make(chan string),
}

var RVExtensionArgsChannels = map[string]chan []string{
	":RESET:":   make(chan []string),
	":MISSION:": make(chan []string),
	":WIN:":     make(chan []string),
	":PLAYER:":  make(chan []string),
	":KILL:":    make(chan []string),
	":SHOT:":    make(chan []string),
	":HIT:":     make(chan []string),
	":EXPORT:":  make(chan []string),
	":FPS:":     make(chan []string),
}

var a3ErrorChan = make(chan error)

func init() {
	a3interface.SetVersion("1.0.0")
	a3interface.RegisterRvExtensionArgsChannels(RVExtensionArgsChannels)

	webhookURL := "YOUR_WEBHOOK_URL"

	// Create the payload with the message
	payload := DiscordPayload{
		Content: "Hello World!",
	}

	// Convert the payload to JSON format
	jsonData, err := json.Marshal(payload)
	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return
	}

	// Create a POST request to the Discord webhook URL
	req, err := http.Post(webhookURL, "application/json", bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}

	// Close the request body
	defer req.Body.Close()

	// Check the response status code
	if req.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", req.Status)
		return
	}
}

func main() {
	fmt.Scanln()
}
