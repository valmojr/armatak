params ["_unit"];

_digitalPointer = laserTarget _unit;

if (!isNull _digitalPointer) then {
	_digitalPointerPosition = _digitalPointer call armatak_fnc_extract_position;

	_link_uid = [_unit] call armatak_fnc_extract_uuid;
	_contact_callsign = ([player] call armatak_fnc_extract_callsign) + ".DP1";

	_dpCot = [_link_uid, _contact_callsign, _digitalPointerPosition select 0, _digitalPointerPosition select 1, _digitalPointerPosition select 2];

	"armatak" callExtension ["tcp_socket:send_digital_pointer_cot", [_dpCot]];
};