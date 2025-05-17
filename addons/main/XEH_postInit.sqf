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

	if (_name == "armatak_tcp_socket") then {
		[_function, "success", _name] call armatak_main_fnc_notify;
	};

	if (_name == "armatak_tcp_socket_error") then {
		[_function, "error", _name] call armatak_main_fnc_notify;
	};

	if (_name == "armatak_video") then {
		[_function, "success", _name] call armatak_main_fnc_notify;
	};

	if (_name == "armatak_video_error") then {
		[_function, "error", _name] call armatak_main_fnc_notify;
	};
}];