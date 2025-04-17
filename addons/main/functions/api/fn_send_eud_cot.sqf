// function name: armatak_fnc_extract_eud_cot_info
// function author: Valmo
// function description: Gets the information necessary for generating the EUD Cursor Over Time

params ["_unit", "_callsign", "_group_name", "_group_role"];

_position = _unit call armatak_fnc_extract_position;
_uuid = _unit call armatak_fnc_extract_uuid;

_eud_cot = [_uuid, _position select 0, _position select 1, _position select 2, _callsign, _group_name, _group_role, _position select 3, speed player / 3.6];

"armatak" callExtension ["cot_router:send_eud_cot", [_eud_cot]];