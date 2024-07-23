// function name: armatak_fnc_extract_info
// function author: Valmo
// function description: Receives a player's unit as param and return the information array needed to send the HTTP request 

params["_unit"];

private _atak_uid = getPlayerUID _unit;
private _atak_latitude = getPos _unit select 0;
private _atak_longitude = getPos _unit select 1;
private _atak_bearing = getDir _unit;
private _atak_role = [_unit] call armatak_fnc_extract_role;
private _atak_callsign = [_unit] call armatak_fnc_extract_callsign;

/*
desired object type
{
  "longitude": -77.0104,
  "latitude": 38.889,
  "attitude": "hostile",
  "bearing": 132, getDir function
  "geoObject": "Gnd Combat Infantry Sniper", extract_role function
  "how": "nonCoT", -- default and won't change
  "name": "Putin",
  "timeout": 600  
}
*/

_unit_info = [_atak_uid, _atak_latitude, _atak_longitude, _atak_bearing, _atak_role, _atak_callsign];

_unit_info