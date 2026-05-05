params ["_drone"];

private _video_url = [_drone] call armatak_fnc_extract_marker_video_url;
if (_video_url == "") exitWith {};

private _uuid = _drone call armatak_fnc_extract_uuid;
private _video_uid = _uuid + "-video";
private _sensor_uid = _uuid + "-sensor";
private _callsign = [_drone] call armatak_fnc_extract_marker_callsign;

private _position = _drone call armatak_client_fnc_extractClientPosition;
private _lat = _position select 1;
private _lon = _position select 2;
private _hae = _position select 3;

private _cameraData = [_drone] call armatak_fnc_extract_uas_camera_data;
private _azimuth = _cameraData select 0;
private _fov = _cameraData select 2;
private _range = _cameraData select 3;

private _payload = [_sensor_uid, _video_uid, _callsign, _lat, _lon, _hae, _azimuth, _fov, _range];
"armatak" callExtension ["tcp_socket:cot:uas_sensor", [_payload]];
