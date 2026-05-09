// function name: armatak_fnc_extract_marker_callsign
// function author: Valmo
// function description: Gets the unit name or classname to be used as TAK callsign in a Marker

params["_unit"];

private _callsign = "";
private _displayName = localize (getText (configOf _unit >> "displayName"));
private _markerCallsignOverride = _unit getVariable ["armatak_attribute_marker_callsign", ""];

if (_markerCallsignOverride isNotEqualTo "") exitWith {
	_markerCallsignOverride
};

if (_displayName isEqualTo "") then {
	_displayName = typeOf _unit;
};

private _vehicleName = vehicleVarName _unit;

if ((([_unit] call BIS_fnc_objectType) select 0) == "Vehicle") then {
	_callsign = [_displayName, _vehicleName] select (_vehicleName isNotEqualTo "");

	if (!isNull driver _unit) then {
		_callsign = _displayName + " | " + ([name (driver _unit)] call armatak_fnc_shorten_name);
	};
};

if (unitIsUAV _unit) then {
	_callsign = [_displayName, _vehicleName] select (_vehicleName isNotEqualTo "");

	private _uavControl = UAVControl _unit;
	private _controller = _uavControl param [0, objNull];
	if (!isNull _controller) then {
		_callsign = _callsign + " | " + ([name _controller] call armatak_fnc_shorten_name);
	};

	if (isUAVConnected _unit) then {
		_callsign = _callsign + " [ON]";
	} else {
		_callsign = _callsign + " [OFF]";
	}
};

if (_callsign isEqualTo "") then {
	_callsign = _displayName;
};

_callsign
