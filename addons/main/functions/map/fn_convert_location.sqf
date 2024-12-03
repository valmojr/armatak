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
	case "Malden": {
		_realLocation = _position call armatak_fnc_convert_to_malden;
	};
	case "VR": {
		_realLocation = _position call armatak_fnc_convert_to_vr;
	};
	default {
		_realLocation = [0, 0, 0];
	};
};

_realLocation