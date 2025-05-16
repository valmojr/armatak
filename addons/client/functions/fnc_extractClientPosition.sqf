#include "..\script_component.hpp"

/*
 * Author: Valmo Trindade
 * This function is used to extract the position of a unit and convert it to a format suitable for SIMTAK.
 *
 * Argument:
 * 0: The first argument <OBJECT> is the unit whose position you want to extract.
 *
 * Return Value:
 * ARRAY -> [latitude, longitude, altitude, bearing]
 *
 * Example:
 * [player] call armatak_client_fnc_extractClientPosition;
 *
 * Public: Yes
 */

params["_unit"];

private _location = (getPos _unit) call armatak_client_fnc_convertClientLocation;

private _atak_latitude = _location select 0;
private _atak_longitude = _location select 1;
private _atak_altitude = _location select 2;
private _atak_bearing = parseNumber ((getDir _unit) toFixed 0);

_unit_info = [_atak_latitude,_atak_longitude,_atak_altitude,_atak_bearing];

_unit_info