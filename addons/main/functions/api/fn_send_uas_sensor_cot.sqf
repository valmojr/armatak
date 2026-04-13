// function name: armatak_fnc_send_uas_sensor_cot
// function author: Valmo / ArmaTAK contributors
// function description:
//   Sends a b-m-p-s-p-loc CoT event every router tick (1 s) for a drone.
//   This is the "sensor position" event consumed by the ATAK UAS Tool to:
//     - Draw the FOV cone on the moving map.
//     - Compute four-corners for AR marker overlay on the video feed.
//     - Show the SPoI (Sensor Point of Interest) crosshair.
//
//   The event references the drone's b-i-v video endpoint via the drone UUID,
//   so armatak_fnc_send_uas_video_cot must also be called for the same drone.
//
//   Exits silently when "armatak_attribute_video_url" is not set, which keeps
//   the behavior identical to the old fn_send_drone_cot for drones without a
//   configured video stream.
//
// Arguments:
//   0: _drone <OBJECT>  The drone object.
//
// Return value: none

params ["_drone"];

private _video_url = _drone getVariable ["armatak_attribute_video_url", ""];
if (_video_url == "") exitWith {};

private _uuid       = _drone call armatak_fnc_extract_uuid;
private _sensor_uid = _uuid + "-sensor";
private _callsign   = [_drone] call armatak_fnc_extract_marker_callsign;

private _pos = (getPos _drone) call armatak_client_fnc_convertClientLocation;
private _lat = _pos select 0;
private _lon = _pos select 1;
private _hae = _pos select 2;

private _azimuth = parseNumber ((getDir _drone) toFixed 0);

private _allTurrets = [_drone, false] call BIS_fnc_allTurrets;
if (count _allTurrets > 0) then {
    private _firstTurretPath = _allTurrets select 0;
    private _turretWeapons   = _drone weaponsTurret _firstTurretPath;
    if (_turretWeapons isNotEqualTo []) then {
        private _tDir = _drone weaponDirection (_turretWeapons select 0);
        if (!((_tDir select 0) == 0 && (_tDir select 1) == 0)) then {
            _azimuth = round (((_tDir select 0) atan2 (_tDir select 1) + 360) mod 360);
        };
    };
};

private _fov = _drone getVariable ["armatak_uas_fov", 60];

private _range = round (((getPosATL _drone) select 2) max 1);

private _payload = [_sensor_uid, _uuid, _callsign, _lat, _lon, _hae, _azimuth, _fov, _range];
"armatak" callExtension ["tcp_socket:cot:uas_sensor", [_payload]];
