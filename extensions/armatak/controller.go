package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os"

	"github.com/indig0fox/a3go/a3interface"
)

var FreeTAKServerURL = os.Getenv("FTS_URL")

func sanitazeArgs(args []string) {
	for i, v := range args {
		args[i] = a3interface.RemoveEscapeQuotes(v)
	}
}

func getRequest(route string) {
	endpoint := FreeTAKServerURL + route

	req, err := http.Get(endpoint)
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}

	defer req.Body.Close()

	if req.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", req.Status)
		return
	}
}

func postRequest(route string, body string) {
	endpoint := FreeTAKServerURL + route

	payload := Payload{
		Content: body,
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return
	}

	req, err := http.Post(endpoint, "application/json", bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}

	defer req.Body.Close()

	if req.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", req.Status)
		return
	}
}

func putRequest(route string, body string) {
	endpoint := FreeTAKServerURL + route

	payload := Payload{
		Content: body,
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return
	}

	req, err := http.Post(endpoint, "application/json", bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}

	defer req.Body.Close()

	if req.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", req.Status)
		return
	}
}

func armatak_controller_ManageAPI_getHelp(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}

func armatak_controller_args_ManageAPI_getHelp(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageGeoObject_postGeoObject(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageGeoObject_postGeoObject(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageGeoObject_putGeoObject(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageGeoObject_putGeoObject(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageGeoObject_getGeoObject(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageGeoObject_getGeoObject(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageGeoObject_getGeoObjectByZone(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageGeoObject_getGeoObjectByZone(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageEmergency_postEmergency(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageEmergency_postEmergency(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageEmergency_getEmergency(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageEmergency_getEmergency(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageEmergency_deleteEmergency(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageEmergency_deleteEmergency(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageChat_postChatToAll(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageChat_postChatToAll(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageRoute_postRoute(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageRoute_postRoute(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManagePresence_postPresence(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManagePresence_postPresence(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManagePresence_putPresence(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManagePresence_putPresence(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageVideoStream_postVideoStream(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageVideoStream_postVideoStream(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_Sensor_postDrone(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_Sensor_postDrone(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_Sensor_postSPI(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_Sensor_postSPI(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}

func armatak_controller_ManageKML_postKML(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "a", nil
}
func armatak_controller_args_ManageKML_postKML(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return "a", nil
}
