#include "script_component.hpp"

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

	switch (_name) do {
		case "UDP SOCKET": {
			[_function, "success", _name] call FUNC(notify);

			switch (_function) do {
				case "EUD Connected": {
					SETVAR(player,EGVAR(client,eudConnected),true);
				};
				case "EUD Disconnected": {
					SETVAR(player,EGVAR(client,eudConnected),false);
				};
				default {};
			};
		};
		case "UDP SOCKET WARNING": {
			[_function, "warning", "UDP Socket"] call FUNC(notify);
		};
		case "UDP SOCKET ERROR": {
			[_function, "error", _name] call FUNC(notify);
			
			if (_function == "UDP Socket is not running") then {
				SETVAR(player,EGVAR(client,eudConnected),false);
			};

			if (_function == "failed to bind UDP socket") then {
				SETVAR(player,EGVAR(client,eudConnected),false);
			};
		};
		case "TCP SOCKET": {
			[_function, "success", _name] call FUNC(notify);
		};
		case "TCP SOCKET ERROR": {
			[_function, "error", _name] call FUNC(notify);
		};
		case "VIDEO": {
			[_function, "success", _name] call FUNC(notify);
		};
		case "VIDEO ERROR": {
			[_function, "error", _name] call FUNC(notify);

			SETVAR(player,EGVAR(video,isStreaming),false);
		};
		default {
			"armatak" callExtension ["log",["error", (_name + _function + _data)]];
		};
	};
}];

GVAR(group_colors) = ["White", "Yellow", "Orange", "Magenta", "Red", "Maroon", "Purple", "DarkBlue", "Blue", "Cyan", "Teal", "Green", "DarkGreen", "Brown"];

missionNamespace setVariable [QGVAR(group_colors), GVAR(group_colors)];