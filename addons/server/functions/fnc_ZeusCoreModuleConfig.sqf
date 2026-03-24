#include "..\script_component.hpp"

params ["_logic"];

_socket_is_running = missionNamespace getVariable ["armatak_tcp_socket_is_running", false];

if (_socket_is_running) exitWith {
	["Socket was called twice", "error", "TCP Socket"] call EFUNC(main,notify);
	closeDialog 1;
};

disableSerialization;

["Connecting to TCP Socket", "success", "TCP Socket"] call EFUNC(main,notify);

_transport_mode = toLower (ctrlText 14006);
_tak_server_instance_address = ctrlText 14000;
_tak_server_instance_port = ctrlText 14001;
_tak_server_tls_name = ctrlText 14002;
_tak_server_tls_ca_cert_path = ctrlText 14003;
_tak_server_tls_client_cert_path = ctrlText 14004;
_tak_server_tls_client_key_path = ctrlText 14005;
_tak_server_enrollment_port = ctrlText 14007;
_tak_server_enrollment_username = ctrlText 14008;
_tak_server_enrollment_password = ctrlText 14009;
_tak_server_enrollment_client_uid = ctrlText 14010;

_tak_server_fulladdress = ((_tak_server_instance_address) + ":" + (_tak_server_instance_port));

missionNamespace setVariable ["armatak_server_instance", _tak_server_fulladdress];
missionNamespace setVariable ["armatak_tcp_socket_is_running", true];

if (_tak_server_tls_name == "") then {
	_tak_server_tls_name = _tak_server_instance_address;
};

switch (_transport_mode) do {
	case "manual_mtls": {
		"armatak" callExtension [
			"tcp_socket:start_mtls",
			[
				_tak_server_fulladdress,
				_tak_server_tls_name,
				_tak_server_tls_ca_cert_path,
				_tak_server_tls_client_cert_path,
				_tak_server_tls_client_key_path
			]
		];
	};
	case "enroll_mtls": {
		"armatak" callExtension [
			"tcp_socket:start_enroll_mtls",
			[
				_tak_server_instance_address,
				_tak_server_tls_name,
				_tak_server_enrollment_port,
				_tak_server_enrollment_username,
				_tak_server_enrollment_password,
				_tak_server_enrollment_client_uid
			]
		];
	};
	default {
		"armatak" callExtension ["tcp_socket:start", [_tak_server_fulladdress]];
	};
};

_syncUnits = [];

missionNamespace setVariable ["armatak_server_syncedUnits", _syncUnits];

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
				case ((_objectType select 0) == "Vehicle"): {
					_atak_type = [_x] call armatak_fnc_extract_role;
					_callsign = [_x] call armatak_fnc_extract_marker_callsign;

					[_x, _atak_type, _callsign] call armatak_fnc_send_marker_cot;

					_x call armatak_fnc_extract_sensor_data;
				};
				case ((_objectType select 0) == "VehicleAutonomous"): {
					_atak_type = [_x] call armatak_fnc_extract_role;
					_callsign = [_x] call armatak_fnc_extract_marker_callsign;

					[_x, _atak_type, _callsign] call armatak_fnc_send_drone_cot;
					[_x] call armatak_fnc_send_digital_pointer_cot;

					_x call armatak_fnc_extract_sensor_data;
				};
			};
		} forEach GVAR(syncedUnits);
	}, 1, []] call CBA_fnc_addPerFrameHandler;
deleteVehicle _logic;
closeDialog 1;
