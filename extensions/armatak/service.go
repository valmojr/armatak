package main

import "strconv"

func armatak_service_ManageAPI_getHelp() (string, error) {
	return getRequest("manageAPI/getHelp") // ERROR - returning unsuported on 2.2 FTS (API is returning this, not my fault)
}

func armatak_service_ManageGeoObject_postGeoObject(args []string) (string, error) {
	longitude, longitudeError := strconv.ParseFloat(args[1], 32)

	if longitudeError != nil {
		return "", longitudeError
	}

	latitude, latitudeError := strconv.ParseFloat(args[2], 32)

	if latitudeError != nil {
		return "", latitudeError
	}

	bearing, bearingError := strconv.Atoi(args[3])

	if bearingError != nil {
		return "", bearingError
	}

	payload := GeoObject{
		UID:       args[0],
		Longitude: float32(longitude),
		Latitude:  float32(latitude),
		Attitude:  "friendly",
		Bearing:   bearing,
		GeoObject: args[4],
		Name:      args[5],
	}

	return postRequest("http://localhost:3000/ManageGeoObject/postGeoObject", payload)
}

func armatak_service_ManageGeoObject_putGeoObject() {}

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
