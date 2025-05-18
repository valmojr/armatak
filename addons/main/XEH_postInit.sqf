#include "script_component.hpp"

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

	if (_name == "WEBSOCKET") then {
		[_function, "success", _name] call FUNC(notify);

		switch (_function) do {
			case "EUD connected": {
				SETVAR(player,EGVAR(client,eudConnected),true);
			};
			case "EUD disconnected": {
				SETVAR(player,GVAR(eudConnected),false);
			};
			default {};
		};
	};

	if (_name == "WEBSOCKET WARNING") then {
		[_function, "warning", "WEBSOCKET"] call FUNC(notify);
	};

	if (_name == "WEBSOCKET ERROR") then {
		[_function, "error", _name] call FUNC(notify);
	};

	if (_name == "TCP SOCKET") then {
		[_function, "success", _name] call FUNC(notify);
	};

	if (_name == "TCP SOCKET ERROR") then {
		[_function, "error", _name] call FUNC(notify);
	};

	if (_name == "VIDEO") then {
		[_function, "success", _name] call FUNC(notify);
	};

	if (_name == "VIDEO ERROR") then {
		[_function, "error", _name] call FUNC(notify);
	};
}];