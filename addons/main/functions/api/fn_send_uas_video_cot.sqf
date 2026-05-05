params ["_drone"];

private _video_url = [_drone] call armatak_fnc_extract_marker_video_url;
if (_video_url == "") exitWith {};

private _uuid = _drone call armatak_fnc_extract_uuid;
private _video_uid = _uuid + "-video";
private _callsign = [_drone] call armatak_fnc_extract_marker_callsign;

private _signature = format ["%1|%2|%3", _video_uid, _callsign, _video_url];
private _nextRefreshAt = _drone getVariable ["armatak_next_uas_video_refresh_at", 0];
private _lastSignature = _drone getVariable ["armatak_last_uas_video_signature", ""];

if (_signature == _lastSignature && {diag_tickTime < _nextRefreshAt}) exitWith {};

_drone setVariable ["armatak_last_uas_video_signature", _signature, false];
_drone setVariable ["armatak_next_uas_video_refresh_at", diag_tickTime + 300, false];

private _payload = [_video_uid, _callsign, _video_url];
"armatak" callExtension ["tcp_socket:cot:uas_video", [_payload]];
