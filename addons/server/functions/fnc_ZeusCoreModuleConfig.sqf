#include "..\script_component.hpp"

params ["_logic"];

_socket_is_running = missionNamespace getVariable ["armatak_tcp_socket_is_running", false];

if (_socket_is_running) exitWith {
	["Socket was called twice", "error", "TCP Socket"] call EFUNC(main,notify);
	closeDialog 1;
};

disableSerialization;

["Connecting to TCP Socket", "success", "TCP Socket"] call EFUNC(main,notify);

_tak_server_instance_address = ctrlText 14000;
_tak_server_instance_port = ctrlText 14001;

_tak_server_fulladdress = ((_tak_server_instance_address) + ":" + (_tak_server_instance_port));

missionNamespace setVariable ["armatak_server_instance", _tak_server_fulladdress];
missionNamespace setVariable ["armatak_tcp_socket_is_running", true];
missionNamespace setVariable ["armatak_group_colors", ["White", "Yellow", "Orange", "Magenta", "Red", "Maroon", "Purple", "DarkBlue", "Blue", "Cyan", "Teal", "Green", "DarkGreen", "Brown"]];

"armatak" callExtension ["tcp_socket:start", [_tak_server_fulladdress]];

_syncUnits = [];

missionNamespace setVariable ["armatak_marked_units", _syncUnits];

GVAR(syncedUnits) = missionNamespace getVariable "armatak_marked_units";

[{
	GVAR(syncedUnits) = missionNamespace getVariable "armatak_marked_units";

	{
		_objectType = _x call BIS_fnc_objectType;
		switch (true) do {
			case ((_objectType select 0) == "Soldier"): {
				_callsign = [_x] call armatak_fnc_extract_unit_callsign;
				_group_name = [group _x] call armatak_fnc_extract_group_color;
				_group_role = [_x] call armatak_fnc_extract_group_role;

				[_x, _callsign, _group_name, _group_role] call armatak_fnc_send_eud_cot;
				[_x] call armatak_fnc_send_digital_pointer_cot;
			};
			case ((_objectType select 0) == "Vehicle"): {
				_atak_type = [_x] call armatak_fnc_extract_role;
				_callsign = [_x] call armatak_fnc_extract_marker_callsign;

				[_x, _atak_type, _callsign] call armatak_fnc_send_marker_cot;
			};
		};
		if (unitIsUAV _x) then {
			[_x] call armatak_fnc_send_drone_cot;
			[_x] call armatak_fnc_send_digital_pointer_cot;
		};
	} forEach GVAR(syncedUnits);
}, 2, []] call CBA_fnc_addPerFrameHandler;
deleteVehicle _logic;
closeDialog 1;