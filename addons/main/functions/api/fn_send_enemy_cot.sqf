// function name: armatak_fnc_send_eud_cot
// function author: Valmo
// function description: Gets the information necessary for generating the EUD Cursor Over Time

params ["_unit"];

_unit_position = _unit call armatak_client_fnc_extractClientPosition; 

_uuid = _unit call armatak_fnc_extract_uuid;
_type = _unit call armatak_fnc_extract_role;
_callsign = _unit call armatak_fnc_extract_marker_callsign;
 
_marker_cot = [_uuid, _type, _unit_position select 1, _unit_position select 2, _unit_position select 3, _callsign, _unit_position select 5, _unit_position select 6]; 
 
"armatak" callExtension ["tcp_socket:send_marker_cot", [_marker_cot]]; 
