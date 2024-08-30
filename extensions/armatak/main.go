package main

import (
	"fmt"

	"github.com/indig0fox/a3go/a3interface"
)

var EXTENSION_NAME = "ARMATAK"

var a3ErrorChannel chan []string = make(chan []string)

func main() {
	fmt.Scanln()
}

func init() {
	a3interface.SetVersion("0.0.0")
	a3interface.RegisterErrorChan(a3ErrorChannel)

	a3interface.NewRegistration("get_uid").
		SetDefaultResponse("getting uuid4").
		SetRunInBackground(false).
		SetFunction(armatak_controller_get_uid).
		SetArgsFunction(armatak_controller_args_get_uid).
		Register()

	a3interface.NewRegistration("get_auth_token").
		SetDefaultResponse("getting auth token").
		SetRunInBackground(false).
		SetFunction(armatak_controller_get_auth_token).
		SetArgsFunction(armatak_controller_args_get_auth_token).
		Register()

	a3interface.NewRegistration("post_marker").
		SetDefaultResponse("posting marker").
		SetRunInBackground(true).
		SetFunction(armatak_controller_post_marker).
		SetArgsFunction(armatak_controller_args_post_marker).
		Register()

	a3interface.NewRegistration("post_marker_debug").
		SetDefaultResponse("posting marker debug mode").
		SetRunInBackground(false).
		SetFunction(armatak_controller_post_marker_debug).
		SetArgsFunction(armatak_controller_args_post_marker_debug).
		Register()

	a3interface.NewRegistration("delete_marker").
		SetDefaultResponse("deleting marker").
		SetRunInBackground(false).
		SetFunction(armatak_controller_delete_marker).
		SetArgsFunction(armatak_controller_args_delete_marker).
		Register()
}
