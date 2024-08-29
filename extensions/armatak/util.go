package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"

	"github.com/indig0fox/a3go/a3interface"
)

func sanitazeArgs(args []string) {
	for i, v := range args {
		args[i] = a3interface.RemoveEscapeQuotes(v)
	}
}

func invalidCallExtensionMethod(complement string) (string, error) {
	return "Invalid callExtension method" + complement, nil
}

func parseMarkerArgs(args []string) (Marker, error) {
	sanitazeArgs(args)

	latitude, latitudeError := strconv.ParseFloat(args[1], 32)

	if latitudeError != nil {
		return Marker{}, latitudeError
	}

	longitude, longitudeError := strconv.ParseFloat(args[2], 32)

	if longitudeError != nil {
		return Marker{}, longitudeError
	}

	speed, speedError := strconv.ParseFloat(args[3], 32)

	if speedError != nil {
		speed = 0
	}

	bearing, bearingError := strconv.ParseFloat(args[4], 32)

	if bearingError != nil {
		bearing = 0
	}

	altitude, altitudeError := strconv.ParseFloat(args[7], 32)

	if altitudeError != nil {
		altitude = 0
	}

	marker := Marker{
		UID:       args[0],
		Latitude:  latitude,
		Longitude: longitude,
		Speed:     int(speed / 4),
		Course:    int(bearing) + 6,
		Type:      args[5],
		Name:      args[6],
		HAE:       int(altitude),
	}

	return marker, nil
}

func postRequestWithoutToken(route string, body any) ([]byte, error) {
	jsonData, err := json.Marshal(body)
	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return nil, err
	}

	client := &http.Client{}

	req, err := http.NewRequest(http.MethodPost, route, bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return nil, err
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending request:", err)
		return nil, err
	}

	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", resp.Status)
		return nil, err
	}

	parsedBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("getting Error reading response body: %w", err)
	}

	return (parsedBody), nil
}

func postRequest(route string, body any) (string, error) {
	jsonData, err := json.Marshal(body)
	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return "", err
	}

	client := &http.Client{}

	req, err := http.NewRequest(http.MethodPost, route, bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return "", err
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending request:", err)
		return "", err
	}

	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", resp.Status)
		return "", nil
	}

	parsedBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("getting Error reading response body: %w", err)
	}

	return string(parsedBody), nil
}

func deleteRequest(route string, body any) (string, error) {
	jsonData, err := json.Marshal(body)
	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return "", err
	}

	client := &http.Client{}

	req, err := http.NewRequest(http.MethodDelete, route, bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return "", err
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending request:", err)
		return "", err
	}

	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", resp.Status)
		return "", nil
	}

	parsedBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("getting Error reading response body: %w", err)
	}

	return string(parsedBody), nil
}
