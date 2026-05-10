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
					call EFUNC(uav,stopMavlinkBroadcast);
					"armatak" callExtension ["uas:stop_endpoint", []];
					"armatak" callExtension ["mdns:stop", []];
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
				call EFUNC(uav,stopMavlinkBroadcast);
				"armatak" callExtension ["uas:stop_endpoint", []];
				"armatak" callExtension ["mdns:stop", []];
			};

			if (_function == "failed to bind UDP socket") then {
				SETVAR(player,EGVAR(client,eudConnected),false);
				call EFUNC(uav,stopMavlinkBroadcast);
				"armatak" callExtension ["uas:stop_endpoint", []];
				"armatak" callExtension ["mdns:stop", []];
			};
		};
		case "MAVLINK UDP ERROR": {
			_message = _function;
			if (_data isNotEqualTo "") then {
				_message = format ["%1: %2", _function, _data];
			};

			[_message, "warning", _name] call FUNC(notify);
		};
		case "MAVLINK UDP": {
			private _history = missionNamespace getVariable ["armatak_uav_mavlink_callback_history", []];
			_history pushBack [diag_tickTime, _function, _data];
			if ((count _history) > 50) then {
				_history deleteRange [0, (count _history) - 50];
			};
			missionNamespace setVariable ["armatak_uav_mavlink_callback_history", _history];
			missionNamespace setVariable ["armatak_uav_last_mavlink_callback", [diag_tickTime, _function, _data]];

			switch (_function) do {
				case "COMMAND_LONG";
				case "COMMAND_INT";
				case "COMMAND_ACK";
				case "MISSION_COUNT";
				case "MISSION_ITEM";
				case "MISSION_ITEM_INT";
				case "MISSION_CLEAR_ALL";
				case "MISSION_SET_CURRENT";
				case "SET_HOME_POSITION";
				case "SET_MODE";
				case "SET_POSITION_TARGET_GLOBAL_INT";
				case "MANUAL_CONTROL": {
					"armatak" callExtension ["log", [["info", format ["MAVLINK UDP CALLBACK %1 %2", _function, _data]]]];
					[_function, _data] call EFUNC(uav,handleMavlinkCallback);
				};
				default {};
			};
		};
		case "TCP SOCKET": {
			[_function, "success", _name] call FUNC(notify);
		};
		case "TCP SOCKET ERROR": {
			_message = _function;
			if (_data isNotEqualTo "") then {
				_message = format ["%1: %2", _function, _data];
			};

			[_message, "error", _name] call FUNC(notify);
		};
		case "VIDEO": {
			[_function, "success", _name] call FUNC(notify);
		};
		case "MDNS": {
			[_function, "success", _name] call FUNC(notify);
		};
		case "MDNS ERROR": {
			[_function, "warning", _name] call FUNC(notify);
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
