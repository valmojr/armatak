#include "script_component.hpp"

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

	switch (_name) do {
		case "WEBSOCKET": {
			[_function, "success", _name] call FUNC(notify);

			switch (_function) do {
				case "EUD connected": {
					SETVAR(player,EGVAR(client,eudConnected),true);
				};
				case "EUD disconnected": {
					SETVAR(player,EGVAR(client,eudConnected),false);
				};
				default {};
			};
		};
		case "WEBSOCKET WARNING": {
			[_function, "warning", "WEBSOCKET"] call FUNC(notify);
		};
		case "WEBSOCKET ERROR": {
			[_function, "error", _name] call FUNC(notify);

			if (_function == "Websocket is not running") then {
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