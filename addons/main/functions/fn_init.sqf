params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer && _activated) exitWith {
	private _warning = format ["<t color='#FF8021'>ARMATAK</t><br/> %1", "Connecting..."];
	[[_warning, 1.5]] call CBA_fnc_notify;

	_atak_ots_address = _logic getVariable "armatak_module_ots_api_instance_address";
	_atak_ots_protocol = _logic getVariable "armatak_module_ots_api_instance_protocol";
	_atak_ots_port = _logic getVariable "armatak_module_ots_api_instance_port";

	_atak_ots_fulladdress = _atak_ots_protocol + ":" + "/" + "/" + _atak_ots_address + ":" + (str _atak_ots_port);
	_atak_ots_api_username = _logic getVariable "armatak_module_ots_api_instance_username";
	_atak_ots_api_password = _logic getVariable "armatak_module_ots_api_instance_password";

	missionNamespace setVariable ["_atak_server_instance", _atak_ots_fulladdress];
	missionNamespace setVariable ["_atak_server_instance_username", _atak_ots_api_username];
	missionNamespace setVariable ["_atak_server_instance_password", _atak_ots_api_password];

	_atak_server_instance_token = call armatak_fnc_extract_auth_token;

	if (isNull _atak_server_instance_token) then {
		private _warning = format ["<t color='#2B7319'>ARMATAK</t><br/> %1", "Connected"];
		[[_warning, 2]] call CBA_fnc_notify;
	} else {
		private _warning = format ["<t color='#FF0000'>ARMATAK</t><br/> %1", "Connection Failed"];
		[[_warning, 2]] call CBA_fnc_notify;
	};

	if (isMultiplayer) then {
		[{
			[{
				{
					private _unit = _x;
					_unit call armatak_fnc_postMarker;
					_unit call armatak_fnc_postDroneMarker;
				} forEach playableUnits;
			}, 1, []] call CBA_fnc_addPerFrameHandler;
		}, [], 1] call CBA_fnc_waitAndExecute;

		addMissionEventHandler ["MPEnded", {
			{
				private _unit = _x;
				_unit call armatak_fnc_deleteMarker;
			} forEach playableUnits;
		}];
	} else {
		[{
			player call armatak_fnc_postMarker;
			player call armatak_fnc_postDroneMarker;
		}, 1, []] call CBA_fnc_addPerFrameHandler;

		addMissionEventHandler ["Ended", {
			player call armatak_fnc_deleteMarker;
		}];
	};
};