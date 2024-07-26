package main

import (
	"os"

	"github.com/indig0fox/a3go/a3interface"
)

var FreeTAKServerURL = os.Getenv("FTS_URL")

func armatak_controller_ManageAPI_getHelp(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {

	return armatak_service_ManageAPI_getHelp()
}

func armatak_controller_args_ManageAPI_getHelp(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return armatak_service_ManageAPI_getHelp()
}

func armatak_controller_ManageGeoObject_postGeoObject(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "You must use the armatak_fnc_extract_info function as param", nil
}
func armatak_controller_args_ManageGeoObject_postGeoObject(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	sanitazeArgs(args)
	return armatak_service_ManageGeoObject_postGeoObject(args)
}

func armatak_controller_ManageGeoObject_putGeoObject(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return "You must use the armatak_fnc_extract_info function as param", nil
}
func armatak_controller_args_ManageGeoObject_putGeoObject(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return armatak_service_ManageGeoObject_putGeoObject(args)
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
