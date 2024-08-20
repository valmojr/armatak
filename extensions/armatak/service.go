package main

import (
	"encoding/json"

	"github.com/google/uuid"
)

func armatak_service_get_uid() (string, error) {
	return uuid.New().String(), nil
}

func armatak_service_get_auth_token(args []string) (string, error) {
	authInfo := AuthInfo{
		Username: args[1],
		Password: args[2],
	}

	response, responseError := postRequestWithoutToken(args[0]+"/api/login?include_auth_token", authInfo)

	if responseError != nil {
		return "", responseError
	}

	jsonString := string(response)

	var data struct {
		Meta     struct{} `json:"meta"`
		Response struct {
			CSRFToken string `json:"csrf_token"`
			User      struct {
				AuthenticationToken string `json:"authentication_token"`
			} `json:"user"`
		} `json:"response"`
	}

	err := json.Unmarshal([]byte(jsonString), &data)
	if err != nil {
		return "", err
	}

	authToken := data.Response.User.AuthenticationToken

	return authToken, nil
}

func armatak_service_post_marker(args []string) (string, error) {
	marker, markerError := parseMarkerArgs(args)

	if markerError != nil {
		return "", markerError
	}

	response, responseError := postRequest(args[8]+"/api/markers?auth_token="+args[9], marker)

	if responseError != nil {
		return "", responseError
	}

	return response, nil
}
