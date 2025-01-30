// function name: armatak_fnc_extract_human_cot_info
// function author: Valmo
// function description: Gets the information necessary for generating the Human Cursor Over Time

params ["_unit", "_callsign", "_group_name", "_group_role"];

_unit_position = _unit call armatak_fnc_extract_position;
_uuid = _unit call armatak_fnc_extract_uuid;

_human_cot = [_uuid, _unit_position select 0, _unit_position select 1, _unit_position select 2, _callsign, _group_name, _group_role, _unit_position select 3, speed player / 3.6];

"armatak" callExtension ["cot_router:send_human_cot", [_human_cot]];