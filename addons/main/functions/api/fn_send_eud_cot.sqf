// function name: armatak_fnc_send_eud_cot
// function author: Valmo
// function description: Gets the information necessary for generating the EUD Cursor Over Time

params ["_unit", "_callsign", "_group_name", "_group_role"];

_position = _unit call armatak_client_fnc_extractClientPosition;

_uuid = _unit call armatak_fnc_extract_uuid;

_eud_cot = [_uuid, _position select 1, _position select 2, _position select 3, _callsign, _group_name, _group_role, _position select 5, _position select 6];
"armatak" callExtension ["tcp_socket:cot:eud", [_eud_cot]];
