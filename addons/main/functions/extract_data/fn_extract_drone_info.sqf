params["_drone"];
private _location = (getPos _drone) call armatak_fnc_convert_location;
private _atak_uid = _drone call armatak_fnc_extract_uuid;
private _atak_latitude = _location select 0;
private _atak_longitude = _location select 1;
private _atak_speed = speed _drone;
private _atak_bearing = parseNumber ((getDir _drone) toFixed 0);
private _atak_role = "a-f-A";
private _atak_callsign = _drone getVariable "_atak_uav_callsign";
private _atak_server_instance = missionNamespace getVariable "_atak_server_instance";
private _atak_server_instance_token = missionNamespace getVariable "_atak_server_instance_token";
private _atak_altitude = _location select 2;
switch (side _drone) do {
	case "WEST": {
		_atak_role = "a-f-A-M-F-Q"
	};
	case "EAST": {
		_atak_role = "a-h-A-M-F-Q"
	};
	case "INDEPENDENT": {
		_atak_role = "a-n-A-M-F-Q"
	};
	case "CIVILIAN": {
		_atak_role = "a-f-A-C"
	};
	default {
		_atak_role = "a-f-A-M-F-Q"
	};
};
_drone_info = [_atak_uid, _atak_latitude, _atak_longitude, _atak_speed, _atak_bearing, _atak_role, _atak_callsign, _atak_altitude, _atak_server_instance, _atak_server_instance_token];

_drone_info