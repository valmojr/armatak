package main

import (
	"strconv"
)

func armatak_service_ManageAPI_getHelp() (string, error) {
	return getRequest("manageAPI/getHelp") // ERROR - returning unsuported on 2.2 FTS (API is returning this, not my fault)
}

func armatak_service_ManageGeoObject_postGeoObject(args []string) (string, error) {
	sanitazeArgs(args)

	latitude, latitudeError := strconv.ParseFloat(args[1], 32)

	if latitudeError != nil {
		return "", latitudeError
	}

	longitude, longitudeError := strconv.ParseFloat(args[2], 32)

	if longitudeError != nil {
		return "", longitudeError
	}

	bearing, bearingError := strconv.ParseFloat(args[4], 32)

	if bearingError != nil {
		bearing = 0
	}

	payload := GeoObject{
		UID:       args[0],
		Longitude: longitude,
		Latitude:  latitude,
		Attitude:  args[3],
		Bearing:   int(bearing),
		GeoObject: args[5],
		How:       "nonCoT",
		Name:      args[6],
		Timeout:   600,
	}

	return postRequest(args[7]+"/ManageGeoObject/postGeoObject", payload, args[8])
}

func armatak_service_ManageGeoObject_putGeoObject(args []string) (string, error) {
	sanitazeArgs(args)

	latitude, latitudeError := strconv.ParseFloat(args[1], 32)

	if latitudeError != nil {
		return "", latitudeError
	}

	longitude, longitudeError := strconv.ParseFloat(args[2], 32)

	if longitudeError != nil {
		return "", longitudeError
	}

	bearing, bearingError := strconv.ParseFloat(args[4], 32)

	if bearingError != nil {
		bearing = 0
	}

	uid := ""

	if args[0] != "<null>" {
		uid = args[0]
	}

	payload := GeoObject{
		UID:       uid,
		Longitude: longitude,
		Latitude:  latitude,
		Attitude:  args[3],
		Bearing:   int(bearing),
		GeoObject: args[5],
		How:       "nonCoT",
		Name:      args[6],
		Timeout:   600,
	}

	return putRequest(args[7]+"/ManageGeoObject/putGeoObject", payload, args[8])
}

func armatak_service_ManageGeoObject_getGeoObject() {}

func armatak_service_ManageGeoObject_getGeoObjectByZone() {}

func armatak_service_ManageEmergency_postEmergency() {}

func armatak_service_ManageEmergency_getEmergency() {}

func armatak_service_ManageEmergency_deleteEmergency() {}

func armatak_service_ManageChat_postChatToAll() {}

func armatak_service_ManageRoute_postRoute() {}

func armatak_service_ManagePresence_postPresence() {}

func armatak_service_ManagePresence_putPresence() {}

func armatak_service_ManageVideoStream_postVideoStream() {}

func armatak_service_Sensor_postDrone() {}

func armatak_service_Sensor_postSPI() {}

func armatak_service_ManageKML_postKML() {}
