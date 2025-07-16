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

private _uuid = _unit call armatak_fnc_extract_uuid;
private _pos = (getPos _unit) call FUNC(convertClientLocation);
private _callsign = _unit call armatak_fnc_extract_unit_callsign;
private _bearing = parseNumber ((getDir _unit) toFixed 0);
private _speed = speed _unit / 3.6;

_payload = [_uuid, _pos select 0, _pos select 1, _pos select 2, _callsign, _bearing, _speed, _callsign];

_payload
