#include "script_component.hpp"

missionNamespace setVariable ["have_this_runned", true];

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

	if (_name == "WEBSOCKET") then {
		[_function, "success", _name] call armatak_main_fnc_notify;
	};

	if (_name == "WEBSOCKET WARNING") then {
		[_function, "warning", "WEBSOCKET"] call armatak_main_fnc_notify;
	};

	if (_name == "WEBSOCKET ERROR") then {
		[_function, "error", _name] call armatak_main_fnc_notify;
	};

	if (_name == "TCP SOCKET") then {
		[_function, "success", _name] call armatak_main_fnc_notify;
	};

	if (_name == "TCP SOCKET ERROR") then {
		[_function, "error", _name] call armatak_main_fnc_notify;
	};

	if (_name == "VIDEO") then {
		[_function, "success", _name] call armatak_main_fnc_notify;
	};

	if (_name == "VIDEO ERROR") then {
		[_function, "error", _name] call armatak_main_fnc_notify;
	};
}];