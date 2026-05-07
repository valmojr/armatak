#include "..\script_component.hpp"

params ["_function", ["_data", "", [""]]];

if (!hasInterface) exitWith {};

private _payload = [_data] call FUNC(parseMavlinkCallbackData);
private _uav = getConnectedUAV player;
if (isNull _uav) exitWith {
	"armatak" callExtension ["log", [["warn", format ["Ignoring MAVLINK UDP callback %1 because no UAV is connected: %2", _function, _data]]]];
};

private _command = parseNumber (_payload getOrDefault ["command", "-1"]);
private _commandName = _payload getOrDefault ["command_name", "UNKNOWN"];

switch (_function) do {
	case "COMMAND_LONG": {
		switch (_command) do {
			case 400: {
				private _armValue = parseNumber (_payload getOrDefault ["param1", "0"]);
				private _doArm = _armValue >= 1;
				_uav engineOn _doArm;
				if (_doArm) then {
					_uav setFuel ((fuel _uav) max 0.1);
				};
				systemChat format ["ATAK %1 %2", ["DISARM", "ARM"] select _doArm, [_uav] call armatak_fnc_extract_marker_callsign];
				"armatak" callExtension ["log", [["info", format ["Applied MAVLINK COMMAND_LONG %1 (%2) to UAV %3", _command, _commandName, _uav]]]];
			};
			case 22: {
				_uav engineOn true;
				_uav setFuel ((fuel _uav) max 0.1);
				_uav flyInHeight 75;

				if (_uav isKindOf "Helicopter" || {_uav isKindOf "VTOL_Base_F"} || {_uav isKindOf "Quadbike_01_F"}) then {
					private _velocity = velocityModelSpace _uav;
					_uav setVelocityModelSpace [
						_velocity select 0,
						(_velocity select 1) max 15,
						((_velocity select 2) max 0) + 8
					];
				};

				systemChat format ["ATAK TAKEOFF %1", [_uav] call armatak_fnc_extract_marker_callsign];
				"armatak" callExtension ["log", [["info", format ["Applied MAVLINK TAKEOFF to UAV %1", _uav]]]];
			};
			default {
				"armatak" callExtension ["log", [["info", format ["Unhandled MAVLINK COMMAND_LONG %1 (%2): %3", _command, _commandName, _data]]]];
			};
		};
	};
	case "COMMAND_INT": {
		"armatak" callExtension ["log", [["info", format ["Received MAVLINK COMMAND_INT %1 (%2): %3", _command, _commandName, _data]]]];
	};
	case "COMMAND_ACK": {
		"armatak" callExtension ["log", [["info", format ["Received MAVLINK COMMAND_ACK %1 (%2): %3", _command, _commandName, _data]]]];
	};
	case "MANUAL_CONTROL": {
		"armatak" callExtension ["log", [["info", format ["Received MAVLINK MANUAL_CONTROL: %1", _data]]]];
	};
	default {
		"armatak" callExtension ["log", [["info", format ["Unhandled MAVLINK UDP callback %1: %2", _function, _data]]]];
	};
};
