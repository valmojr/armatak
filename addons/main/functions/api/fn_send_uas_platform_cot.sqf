params ["_drone"];

private _uuid = _drone call armatak_fnc_extract_uuid;
private _uavControl = UAVControl _drone;
private _controller = _uavControl param [0, objNull];
private _controller_uid = if (!isNull _controller) then { [_controller] call armatak_fnc_extract_uuid } else { _drone getVariable ["armatak_uas_controller_uid", _uuid] };
private _callsign = [_drone] call armatak_fnc_extract_marker_callsign;

private _atak_role = "a-f-A-M-H-Q";
switch (side _drone) do {
    case west: {
        _atak_role = "a-f-A-M-H-Q";
    };
    case east: {
        _atak_role = "a-h-A-M-H-Q";
    };
    case independent: {
        _atak_role = "a-n-A-M-H-Q";
    };
    case civilian: {
        _atak_role = "a-f-A-C";
    };
    default {
        _atak_role = "a-f-A-M-H-Q";
    };
};

private _position = _drone call armatak_client_fnc_extractClientPosition;
private _lat = _position select 1;
private _lon = _position select 2;
private _hae = _position select 3;
private _course = _position select 5;
private _speed = _position select 6;

private _cameraData = [_drone] call armatak_fnc_extract_uas_camera_data;
private _azimuth = _cameraData select 0;
private _elevation = _cameraData select 1;
private _fov = _cameraData select 2;
private _range = _cameraData select 3;
private _vfov = _drone getVariable ["armatak_uas_vfov", _fov];

private _yaw = round (getDir _drone);
private _pitch = (vectorDir _drone) select 2;
private _roll = (vectorUp _drone) select 0;
private _isFlying = parseNumber (isEngineOn _drone);
private _hal = ((getPosATL _drone) select 2) max 0;
private _vehicleType = typeOf _drone;

private _payload = [
    _uuid,
    _atak_role,
    _callsign,
    _lat,
    _lon,
    _hae,
    _course,
    _speed,
    _azimuth,
    _elevation,
    _fov,
    _vfov,
    _range,
    _yaw,
    _pitch,
    _roll,
    _hal,
    _vehicleType,
    _isFlying,
    _controller_uid
];

"armatak" callExtension ["tcp_socket:cot:uas_platform", [_payload]];
