// function name: armatak_fnc_extract_info
// function author: Valmo
// function description: Receives a player's unit as param and return the information array needed to send the HTTP request 

params["_unit"];
private _location = (getPos _unit) call armatak_fnc_convert_location;

private _atak_uid = [_unit] call armatak_fnc_extract_uuid;
private _atak_latitude = _location select 0;
private _atak_longitude = _location select 1;
private _atak_speed = speed _unit;
private _atak_bearing = parseNumber ((getDir _unit) toFixed 0);
private _atak_role = [_unit] call armatak_fnc_extract_role;
private _atak_callsign = [_unit] call armatak_fnc_extract_callsign;
private _atak_altitude = _location select 2;
private _atak_server_instance = missionNamespace getVariable "_atak_server_instance";
private _atak_server_instance_token = missionNamespace getVariable "_atak_server_instance_token";
private _atak_server_instance_username = missionNamespace getVariable "_atak_server_instance_username";
private _atak_server_instance_password = missionNamespace getVariable "_atak_server_instance_password";

_drone = vehicle (getConnectedUAVUnit _unit);

if (!isNull _drone) then {
	_drone setVariable ["_atak_uav_connected", true];
	_drone setVariable ["_atak_uav_callsign", "[UAV]" + name _unit];
};

_unit_info = [_atak_uid, _atak_latitude, _atak_longitude, _atak_speed, _atak_bearing, _atak_role, _atak_callsign, _atak_altitude, _atak_server_instance, _atak_server_instance_token, _atak_server_instance_username, _atak_server_instance_password];

_unit_info