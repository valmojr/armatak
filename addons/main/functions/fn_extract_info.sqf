// function name: armatak_fnc_extract_info
// function author: Valmo
// function description: Receives a player's unit as param and return the information array needed to send the HTTP request 

params["_unit"];
private _location = [_unit] call armatak_fnc_convert_location;

private _atak_uid = getPlayerUID _unit;
private _atak_latitude = _location select 0;
private _atak_longitude = _location select 1;
private _atak_side = [_unit] call armatak_fnc_extract_side;
private _atak_bearing = getDir _unit;
private _atak_role = [_unit] call armatak_fnc_extract_role;
private _atak_callsign = [_unit] call armatak_fnc_extract_callsign;

_unit_info = [_atak_uid, _atak_latitude, _atak_longitude, _atak_side, _atak_bearing, _atak_role, _atak_callsign];

_unit_info