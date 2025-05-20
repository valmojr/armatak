#include "..\script_component.hpp"

params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer) exitWith {
	["Connecting to TCP Socket", "success", "TCP Socket"] call EFUNC(main,notify);

	_tak_server_instance_address = _logic getVariable QGVAR(moduleInstanceAddress);
	_tak_server_instance_port = _logic getVariable QGVAR(moduleInstancePort);

	_tak_server_fulladdress = _tak_server_instance_address + ":" + (str _tak_server_instance_port);

	missionNamespace setVariable ["armatak_server_instance", _tak_server_fulladdress];
	missionNamespace setVariable ["armatak_tcp_socket_is_running", true];
	missionNamespace setVariable ["armatak_group_colors", ["White", "Yellow", "Orange", "Magenta", "Red", "Maroon", "Purple", "DarkBlue", "Blue", "Cyan", "Teal", "Green", "DarkGreen", "Brown"]];

	"armatak" callExtension ["tcp_socket:start", [_tak_server_fulladdress]];

	_syncUnits = synchronizedObjects _logic;

	missionNamespace setVariable ["armatak_marked_units", _syncUnits];

	_syncedUnits = missionNamespace getVariable "armatak_marked_units";

	[{
		_syncedUnits = missionNamespace getVariable "armatak_marked_units";

		{
			_objectType = _x call BIS_fnc_objectType;
			if ((_objectType select 0) == "Soldier") then {
				if (!GETVAR(_x,EGVAR(client,eudConnected),false)) then {
					_callsign = [_x] call armatak_fnc_extract_unit_callsign;
					_group_name = [group _x] call armatak_fnc_extract_group_color;
					_group_role = [_x] call armatak_fnc_extract_group_role;

					[_x, _callsign, _group_name, _group_role] call armatak_fnc_send_eud_cot;
					[_x] call armatak_fnc_send_digital_pointer_cot;
				};
			};
			if ((_objectType select 0) == "Vehicle") then {
				_atak_type = [_x] call armatak_fnc_extract_role;
				_callsign = [_x] call armatak_fnc_extract_marker_callsign;

				[_x, _atak_type, _callsign] call armatak_fnc_send_marker_cot;
			};
			if (unitIsUAV _x) then {
				[_x] call armatak_fnc_send_drone_cot;
				[_x] call armatak_fnc_send_digital_pointer_cot;
			};
		} forEach _syncedUnits;
	}, 2, []] call CBA_fnc_addPerFrameHandler;
};

true;