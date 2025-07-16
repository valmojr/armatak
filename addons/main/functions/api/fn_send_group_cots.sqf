// function name: armatak_fnc_send_group_cots
// function author: Valmo
// function description: handle the cot routing for drones

params["_group"];

{
	_callsign = [_x] call armatak_fnc_extract_unit_callsign;
	_group_name = [_group] call armatak_fnc_extract_group_color;
	_group_role = [_x] call armatak_fnc_extract_group_role;

	[_x, _callsign, _group_name, _group_role] call armatak_fnc_send_eud_cot;
} forEach (units _group);
