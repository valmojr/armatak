params["_drone"];

private _location = [_drone] call armatak_fnc_convert_location;

private _atak_uid = [_drone] call armatak_fnc_extract_uuid;
private _atak_latitude = _location select 0;
private _atak_longitude = _location select 1;
private _atak_speed = speed _drone;
private _atak_bearing = parseNumber ((getDir _drone) toFixed 0);
private _atak_role = [_drone] call armatak_fnc_extract_role;
private _atak_callsign = "[UAV]" + typeOf _drone;
private _atak_server_instance = missionNamespace getVariable "_atak_server_instance";
private _atak_server_instance_token = missionNamespace getVariable "_atak_server_instance_token";

_drone_info = [_atak_uid, _atak_latitude, _atak_longitude, _atak_speed, _atak_bearing, _atak_role, _atak_callsign, _atak_server_instance, _atak_server_instance_token];

_drone_info