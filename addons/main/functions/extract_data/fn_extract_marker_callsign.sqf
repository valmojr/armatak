// function name: armatak_fnc_extract_marker_callsign
// function author: Valmo
// function description: Gets the unit name or classname to be used as TAK callsign in a Marker

params["_unit"];

private _callsign = "";
private _displayName = localize (getText (configOf _unit >> "displayName"));

if (_displayName isEqualTo "") then {
	_displayName = typeOf _unit;
};

if ((([_unit] call BIS_fnc_objectType) select 0) == "Vehicle") then {
	_callsign = _displayName;

	if (!isNull driver _unit) then {
		_callsign = _displayName + " | " + ([name (driver _unit)] call armatak_fnc_shorten_name);
	};
};

if (unitIsUAV _unit) then {
	_callsign = _displayName;

	if (isUAVConnected _unit) then {
		_callsign = _callsign + " [ON]";
	} else {
		_callsign = _callsign + " [OFF]";
	}
};

private _markerCallsignOverride = _unit getVariable ["armatak_attribute_marker_callsign", ""];

if (_markerCallsignOverride isNotEqualTo "") then {
	_callsign = _markerCallsignOverride;
};

_callsign
