// function name: armatak_fnc_send_marker_cot
// function author: Valmo
// function description: Gets the information necessary for generating the Marker Cursor Over Time

params ["_unit", "_type", "_callsign"];

_unit_position = _unit call armatak_client_fnc_extractClientPosition;
_video_url = [_unit] call armatak_fnc_extract_marker_video_url;

_uuid = _unit call armatak_fnc_extract_uuid;

_marker_cot = [_uuid, _type, _unit_position select 1, _unit_position select 2, _unit_position select 3, _callsign, _unit_position select 5, _unit_position select 6, _video_url];

"armatak" callExtension ["tcp_socket:cot:marker", [_marker_cot]];
