params["_unit"];

_target = getSensorTargets (_unit);

{
	_unit = _x select 0;
	_position = _x select 1;
	_status = _x select 2;

	if (isNil {
		_unit getVariable "armatak_current_side"
	}) then {
		_unit setVariable ["armatak_current_side", side _unit];
	};

	if (_status != "destroyed" && !(_unit in armatak_server_syncedUnits)) then {
		_unit_position = _unit call armatak_client_fnc_extractClientPosition;

		_uuid = _unit call armatak_fnc_extract_uuid;
		_type = _unit call armatak_fnc_extract_role;
		_callsign = getText (configOf _unit >> "displayName");

		_marker_cot = [_uuid, _type, _unit_position select 1, _unit_position select 2, _unit_position select 3, _callsign, _unit_position select 5, _unit_position select 6];

		"armatak" callExtension ["tcp_socket:send_marker_cot", [_marker_cot]];
	};
} forEach _target;