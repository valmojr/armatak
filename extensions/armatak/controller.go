package main

import (
	"github.com/indig0fox/a3go/a3interface"
)

func armatak_controller_get_uid(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return armatak_service_get_uid()
}

func armatak_controller_args_get_uid(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	return armatak_service_get_uid()
}

func armatak_controller_get_auth_token(
	ctx a3interface.ArmaExtensionContext,
	command string,
) (string, error) {
	return invalidCallExtensionMethod("the auth informations must be provided")
}

func armatak_controller_args_get_auth_token(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	sanitazeArgs(args)

	return armatak_service_get_auth_token(args)
}

func armatak_controller_post_marker(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return invalidCallExtensionMethod("post marker requires args")
}

func armatak_controller_args_post_marker(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	sanitazeArgs(args)

	return armatak_service_post_marker(args)
}

func armatak_controller_post_marker_debug(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return invalidCallExtensionMethod("post marker requires args")
}

func armatak_controller_args_post_marker_debug(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	sanitazeArgs(args)

	return armatak_service_post_marker(args)
}

func armatak_controller_delete_marker(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	return invalidCallExtensionMethod("post marker requires args")
}

func armatak_controller_args_delete_marker(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	sanitazeArgs(args)

	return armatak_service_delete_marker(args)
}
