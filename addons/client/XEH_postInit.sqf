#include "script_component.hpp"

if (!hasInterface) exitWith {};

_local_address = "armatak" callExtension ["local_ip", []] select 0;

CALLEXT(websocket:start);

SETVAR(player,GVAR(localAddress),_local_address);

[{
	"armatak" callExtension ["websocket:location", [player call FUNC(extractClientPosition)]];
}, 1, []] call CBA_fnc_addPerFrameHandler;

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

	if (_name == "WEBSOCKET") then {
		switch (_function) do {
			case "EUD connected": {
				SETVAR(player,GVAR(eudConnected),true);
			};
			case "EUD disconnected": {
				SETVAR(player,GVAR(eudConnected),false);
			};
			default { };
		};
	};

	if (_name == "WEBSOCKET WARNING") then {
		[_function, "warning", "WEBSOCKET"] call armatak_main_fnc_notify;
	};

	if (_name == "WEBSOCKET ERROR") then {
		[_function, "error", _name] call armatak_main_fnc_notify;
	};
}];
