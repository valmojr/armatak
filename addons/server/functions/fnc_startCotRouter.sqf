#include "..\script_component.hpp"

params [["_server_instance", "", [""]]];

missionNamespace setVariable ["armatak_server_instance", _server_instance];
missionNamespace setVariable ["armatak_tcp_socket_is_running", true];

if (isNil { missionNamespace getVariable "armatak_server_syncedUnits" }) then {
	missionNamespace setVariable ["armatak_server_syncedUnits", []];
};

GVAR(syncedUnits) = missionNamespace getVariable "armatak_server_syncedUnits";

[{
	GVAR(syncedUnits) = missionNamespace getVariable "armatak_server_syncedUnits";

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
			case (unitIsUAV _x): {
				if !(_x getVariable ["armatak_uav_mavlink_broadcasting", false]) then {
					_atak_type = [_x] call armatak_fnc_extract_role;
					_callsign = [_x] call armatak_fnc_extract_marker_callsign;

					[_x, _atak_type, _callsign] call armatak_fnc_send_drone_cot;
					[_x] call armatak_fnc_send_digital_pointer_cot;
					_x call armatak_fnc_extract_sensor_data;
				};
			};
			case ((_objectType select 0) == "Vehicle"): {
				_atak_type = [_x] call armatak_fnc_extract_role;
				_callsign = [_x] call armatak_fnc_extract_marker_callsign;

				[_x, _atak_type, _callsign] call armatak_fnc_send_marker_cot;
				_x call armatak_fnc_extract_sensor_data;
			};
			case ((_objectType select 0) == "VehicleAutonomous"): {
				if !(_x getVariable ["armatak_uav_mavlink_broadcasting", false]) then {
					_atak_type = [_x] call armatak_fnc_extract_role;
					_callsign = [_x] call armatak_fnc_extract_marker_callsign;

					[_x, _atak_type, _callsign] call armatak_fnc_send_drone_cot;
					[_x] call armatak_fnc_send_digital_pointer_cot;
					_x call armatak_fnc_extract_sensor_data;
				};
			};
		};
	} forEach GVAR(syncedUnits);
}, 1, []] call CBA_fnc_addPerFrameHandler;

true
