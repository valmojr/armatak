params["_unit"];

_location = null;

switch (worldName) do {
	case "Altis": {
		_location = [_unit] call armatak_fnc_convert_to_altis;
	};
	default {};
};

_location