// function name: armatak_fnc_extract_callsign
// function author: Valmo
// function description: Gets the unit name or classname to be used as TAK callsign

params["_unit"];
private _callsign = "";
if (roleDescription _unit != "") then {
	_callsign = name _unit + " | " + roleDescription _unit;
} else {
	_callsign = name _unit;

	if (_callsign == "Error: No unit") then {
		_callsign = getText(configFile >> "CfgVehicles" >> typeOf _unit >> "displayName");
	};
};

_callsign