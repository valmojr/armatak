#include "..\script_component.hpp"

params ["_message", "_type", ["_title", "ARMATAK"]];

switch (_type) do {
	case "success": {
		_warning = format ["<t color='#00FF21'>%1</t><br/> %2", _title, _message];
		[[_warning, 1.5]] call CBA_fnc_notify;
	};
	case "warning": {
		_warning = format ["<t color='#ffff16'>%1</t><br/> %2", _title, _message];
		[[_warning, 1.5]] call CBA_fnc_notify;
	};
	case "error": {
		_warning = format ["<t color='#FF0021'>%1</t><br/> %2", _title, _message];
		[[_warning, 1.5]] call CBA_fnc_notify;
	};
	default {
		systemChat format ["%1 - %2", _title, _message];
	};
};