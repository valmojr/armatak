// function name: armatak_fnc_extract_callsign
// function author: Valmo
// function description: Gets the unit name or classname to be used as TAK callsign

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

_atak_pre_defined_callsign = _unit getVariable "_atak_callsign";

if (!isNil "_atak_pre_defined_callsign") then {
	_callsign = _atak_pre_defined_callsign;
};

_callsign