// function name: armatak_fnc_extract_marker_callsign
// function author: Valmo
// function description: Gets the unit name or classname to be used as TAK callsign in a Marker

params["_unit"];

private _callsign = "";

if ((([_unit] call BIS_fnc_objectType) select 0) == "Vehicle") then {
	_callsign = getText (configFile >> "CfgVehicles" >> typeOf _unit >> "displayName");

	if (!isNull driver _unit) then {
		_callsign = getText (configFile >> "CfgVehicles" >> typeOf _unit >> "displayName") + " | " + ([name (driver _unit)] call armatak_fnc_shorten_name);
	};
};

if (unitIsUAV _unit) then {
	_callsign = getText (configFile >> "CfgVehicles" >> typeOf _unit >> "displayName");

	if (isUAVConnected _unit) then {
		_callsign = (_callsign) + "[ON]";
	} else {
		_callsign = (_callsign) + "[OFF]";
	}
};

armatak_attribute_marker_callsign = _unit getVariable "armatak_attribute_marker_callsign";

if (!isNil "armatak_attribute_marker_callsign" or armatak_attribute_marker_callsign != '') then {
	_callsign = armatak_attribute_marker_callsign;
};

_callsign