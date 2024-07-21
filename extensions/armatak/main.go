package main

import (
	"fmt"
	"path"

	"github.com/indig0fox/a3go/a3interface"
	"github.com/indig0fox/a3go/assemblyfinder"
	"github.com/joho/godotenv"
)

var modulePath string = assemblyfinder.GetModulePath()
var modulePathDir string = path.Dir(modulePath)

var EXTENSION_NAME = "ARMATAK"

var a3ErrorChannel chan []string = make(chan []string)

func main() {
	fmt.Scanln()
}

func init() {
	fmt.Printf("Module Path Directory => " + modulePathDir)

	godotenv.Load(".env")

	a3interface.SetVersion("0.0.0")
	a3interface.RegisterErrorChan(a3ErrorChannel)

	a3interface.NewRegistration("ManageAPI/getHelp").
		SetDefaultResponse("[Requested Get Help Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageAPI_getHelp).
		SetArgsFunction(armatak_controller_args_ManageAPI_getHelp).
		Register()

	a3interface.NewRegistration("ManageGeoObject/postGeoObject").
		SetDefaultResponse("[Requested postGeoObject Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageGeoObject_postGeoObject).
		SetArgsFunction(armatak_controller_args_ManageGeoObject_postGeoObject).
		Register()

	a3interface.NewRegistration("ManageGeoObject/putGeoObject").
		SetDefaultResponse("[Requested putGeoObject Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageGeoObject_putGeoObject).
		SetArgsFunction(armatak_controller_args_ManageGeoObject_putGeoObject).
		Register()

	a3interface.NewRegistration("ManageGeoObject/getGeoObject").
		SetDefaultResponse("[Requested getGeoObject Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageGeoObject_getGeoObject).
		SetArgsFunction(armatak_controller_args_ManageGeoObject_getGeoObject).
		Register()

	a3interface.NewRegistration("ManageGeoObject/getGeoObjectByZone").
		SetDefaultResponse("[Requested getGeoObjectByZone Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageGeoObject_getGeoObjectByZone).
		SetArgsFunction(armatak_controller_args_ManageGeoObject_getGeoObjectByZone).
		Register()

	a3interface.NewRegistration("ManageEmergency/postEmergency").
		SetDefaultResponse("[Requested postEmergency Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageEmergency_postEmergency).
		SetArgsFunction(armatak_controller_args_ManageEmergency_postEmergency).
		Register()

	a3interface.NewRegistration("ManageEmergency/getEmergency").
		SetDefaultResponse("[Requested getEmergency Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageEmergency_getEmergency).
		SetArgsFunction(armatak_controller_args_ManageEmergency_getEmergency).
		Register()

	a3interface.NewRegistration("ManageEmergency/deleteEmergency").
		SetDefaultResponse("[Requested deleteEmergency Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageEmergency_deleteEmergency).
		SetArgsFunction(armatak_controller_args_ManageEmergency_deleteEmergency).
		Register()

	a3interface.NewRegistration("ManageChat/postChatToAll").
		SetDefaultResponse("[Requested postChatToAll Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageChat_postChatToAll).
		SetArgsFunction(armatak_controller_args_ManageChat_postChatToAll).
		Register()

	a3interface.NewRegistration("ManageRoute/postRoute").
		SetDefaultResponse("[Requested postRoute Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageRoute_postRoute).
		SetArgsFunction(armatak_controller_args_ManageRoute_postRoute).
		Register()

	a3interface.NewRegistration("ManagePresence/postPresence").
		SetDefaultResponse("[Requested postPresence Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManagePresence_postPresence).
		SetArgsFunction(armatak_controller_args_ManagePresence_postPresence).
		Register()

	a3interface.NewRegistration("ManagePresence/putPresence").
		SetDefaultResponse("[Requested putPresence Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManagePresence_putPresence).
		SetArgsFunction(armatak_controller_args_ManagePresence_putPresence).
		Register()

	a3interface.NewRegistration("ManageVideoStream/postVideoStream").
		SetDefaultResponse("[Requested postVideoStream Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageVideoStream_postVideoStream).
		SetArgsFunction(armatak_controller_args_ManageVideoStream_postVideoStream).
		Register()

	a3interface.NewRegistration("Sensor/postDrone").
		SetDefaultResponse("[Requested postDrone Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_Sensor_postDrone).
		SetArgsFunction(armatak_controller_args_Sensor_postDrone).
		Register()

	a3interface.NewRegistration("Sensor/postSPI").
		SetDefaultResponse("[Requested postSPI Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_Sensor_postSPI).
		SetArgsFunction(armatak_controller_args_Sensor_postSPI).
		Register()

	a3interface.NewRegistration("ManageKML/postKML").
		SetDefaultResponse("[Requested postKML Command, starting background process]").
		SetRunInBackground(false).
		SetFunction(armatak_controller_ManageKML_postKML).
		SetArgsFunction(armatak_controller_args_ManageKML_postKML).
		Register()

}
