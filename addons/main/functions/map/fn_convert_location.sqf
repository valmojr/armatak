params["_latitude", "_longitude", "_altitude"];

_position = [_latitude, _longitude, _altitude];

_realLocation = null;

switch (worldName) do {
	case "Altis": {
		_realLocation = _position call armatak_fnc_convert_to_altis;
	};
	case "Stratis": {
		_realLocation = _position call armatak_fnc_convert_to_stratis;
	};
	default {};
};

_realLocation