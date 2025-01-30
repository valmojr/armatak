// function name: armatak_fnc_extract_drone
// function author: Valmo
// function description: Gets the drone information for the CoT Router

params["_drone"];

private _atak_role = "a-f-A";
private _atak_callsign = getText(configFile >> "CfgVehicles" >> typeOf _drone >> "displayName");

switch (side _drone) do {
	case "WEST": {
		_atak_role = "a-f-A-M-F-Q"
	};
	case "EAST": {
		_atak_role = "a-h-A-M-F-Q"
	};
	case "INDEPENDENT": {
		_atak_role = "a-n-A-M-F-Q"
	};
	case "CIVILIAN": {
		_atak_role = "a-f-A-C"
	};
	default {
		_atak_role = "a-f-A-M-F-Q"
	};
};

_cot = [_drone, _atak_role, _atak_callsign] call armatak_fnc_extract_marker_cot_info;

"armatak" callExtension ["cot_router:send_marker_cot", [_cot]];