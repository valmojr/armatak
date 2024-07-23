package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/indig0fox/a3go/a3interface"
)

func sanitazeArgs(args []string) {
	for i, v := range args {
		args[i] = a3interface.RemoveEscapeQuotes(v)
	}
}

func getRequest(route string) (string, error) {
	endpoint := FreeTAKServerURL + route

	req, err := http.Get(endpoint)
	if err != nil {
		return "", fmt.Errorf("getting Error creating request: %w", err)
	}

	defer req.Body.Close()

	if req.StatusCode != http.StatusOK {
		return "", fmt.Errorf("getting Error sending request " + req.Status)
	}

	body, err := io.ReadAll(req.Body)
	if err != nil {
		return "", fmt.Errorf("getting Error reading response body %w", err)
	}

	return string(body), nil
}

func postRequest(route string, body any) (string, error) {
	jsonData, err := json.Marshal(body)

	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return "", err
	}

	req, err := http.Post(route, "application/json", bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return "", err
	}

	defer req.Body.Close()

	if req.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", req.Status)
		return "", nil
	}

	parsedBody, err := io.ReadAll(req.Body)
	if err != nil {
		return "", fmt.Errorf("getting Error reading response body: %w", err)
	}

	return string(parsedBody), nil
}
