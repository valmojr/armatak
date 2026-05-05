params["_unit"];

_target = getSensorTargets (_unit);

{
	private _targetUnit = _x select 0;
	_position = _x select 1;
	_status = _x select 2;
	private _targetType = toLower (typeOf _targetUnit);

	if ((_targetType find "lasertarget") < 0) then {
		if (isNil {
			_targetUnit getVariable "armatak_current_side"
		}) then {
			_targetUnit setVariable ["armatak_current_side", side _targetUnit];
		};

		if (_status != "destroyed" && !(_targetUnit in armatak_server_syncedUnits)) then {
			_unit_position = _targetUnit call armatak_client_fnc_extractClientPosition;

			_uuid = _targetUnit call armatak_fnc_extract_uuid;
			_type = _targetUnit call armatak_fnc_extract_role;
			_callsign = getText (configOf _targetUnit >> "displayName");

			_marker_cot = [_uuid, _type, _unit_position select 1, _unit_position select 2, _unit_position select 3, _callsign, _unit_position select 5, _unit_position select 6];

			"armatak" callExtension ["tcp_socket:cot:marker", [_marker_cot]];
		};
	};
} forEach _target;
