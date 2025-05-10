// function name: armatak_fnc_extract_unit_callsign
// function author: Valmo
// function description: Gets the unit name or classname to be used as TAK callsign from a unit

params["_unit"];

private _callsign = "";

if (roleDescription _unit != "") then {
	_callsign = ([name _unit] call armatak_fnc_shorten_name) + " | " + roleDescription _unit;
} else {
	_callsign = name _unit;

	if (_callsign == "Error: No unit") then {
		_callsign = getText (configFile >> "CfgVehicles" >> typeOf _unit >> "displayName");
	};
};

armatak_attribute_unit_callsign = _unit getVariable "armatak_attribute_unit_callsign";

if (!isNil "armatak_attribute_unit_callsign" or armatak_attribute_unit_callsign != '') then {
	_callsign = armatak_attribute_unit_callsign;
};

_callsign