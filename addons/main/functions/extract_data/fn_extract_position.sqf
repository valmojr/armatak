// function name: armatak_fnc_extract_position
// function author: Valmo
// function description: Receives a player's unit as param and return the information array for SIMTAK

params["_unit"];

private _location = (getPos _unit) call armatak_fnc_convert_location;

private _atak_latitude = _location select 0;
private _atak_longitude = _location select 1;
private _atak_altitude = _location select 2;
private _atak_bearing = parseNumber ((getDir _unit) toFixed 0);

_unit_info = [_atak_latitude,_atak_longitude,_atak_altitude,_atak_bearing];

_unit_info