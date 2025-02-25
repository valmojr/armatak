params["_latitude", "_longitude", "_altitude"];

_position = [_latitude, _longitude, _altitude];

_realLocation = null;

switch (toLower worldName) do {
	case "altis": {
		_realLocation = _position call armatak_fnc_convert_to_altis;
	};
	case "stratis": {
		_realLocation = _position call armatak_fnc_convert_to_stratis;
	};
	case "malden": {
		_realLocation = _position call armatak_fnc_convert_to_malden;
	};
	case "vr": {
		_realLocation = _position call armatak_fnc_convert_to_vr;
	};
	case "cucui": {
		_realLocation = _position call armatak_fnc_convert_to_cucui;
	};
	case "chernarus": {
		_realLocation = _position call armatak_fnc_convert_to_chernarus;
	};
	case "chernarus_summer": {
		_realLocation = _position call armatak_fnc_convert_to_chernarus;
	};
	case "chernarus_winter": {
		_realLocation = _position call armatak_fnc_convert_to_chernarus;
	};
	case "bootcamp_acr": {
		_realLocation = _position call armatak_fnc_convert_to_bukovina;
	};
	case "woodland_acr": {
		_realLocation = _position call armatak_fnc_convert_to_bystrika;
	};
	case "mountains_acr": {
		_realLocation = _position call armatak_fnc_convert_to_takistan_montains;
	};
	case "sara_dbe1": {
		_realLocation = _position call armatak_fnc_convert_to_united_sahrani;
	};
	case "saralite": {
		_realLocation = _position call armatak_fnc_convert_to_united_sahrani;
	};
	case "enoch": {
		_realLocation = _position call armatak_fnc_convert_to_livonia;
	};
	default {
		_warning = format ["<t color='#FF8021'>ARMATAK</t><br/> %1", "Unsupported Map"];
		[[_warning, 1.5]] call CBA_fnc_notify;

		_realLocation = [0, 0, 0];
	};
};

_realLocation