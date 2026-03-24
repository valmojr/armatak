#include "..\script_component.hpp"

params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

	if (isServer) exitWith {
		["Connecting to TCP Socket", "success", "TCP Socket"] call EFUNC(main,notify);

		_transport_mode = _logic getVariable [QGVAR(moduleTransportMode), "tcp"];
		_tak_server_instance_address = _logic getVariable QGVAR(moduleInstanceAddress);
		_tak_server_instance_port = _logic getVariable QGVAR(moduleInstancePort);
		_tak_server_tls_name = _logic getVariable [QGVAR(moduleTlsServerName), ""];
		_tak_server_tls_ca_cert_path = _logic getVariable [QGVAR(moduleTlsCaCertPath), ""];
		_tak_server_tls_client_cert_path = _logic getVariable [QGVAR(moduleTlsClientCertPath), ""];
		_tak_server_tls_client_key_path = _logic getVariable [QGVAR(moduleTlsClientKeyPath), ""];
		_tak_server_enrollment_port = _logic getVariable [QGVAR(moduleEnrollmentPort), 8446];
		_tak_server_enrollment_username = _logic getVariable [QGVAR(moduleEnrollmentUsername), ""];
		_tak_server_enrollment_password = _logic getVariable [QGVAR(moduleEnrollmentPassword), ""];
		_tak_server_enrollment_client_uid = _logic getVariable [QGVAR(moduleEnrollmentClientUid), ""];

		_tak_server_fulladdress = _tak_server_instance_address + ":" + (str _tak_server_instance_port);

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
						str _tak_server_enrollment_port,
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

		_syncUnits = synchronizedObjects _logic;

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
};

true;
