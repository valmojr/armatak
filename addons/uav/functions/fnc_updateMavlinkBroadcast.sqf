#include "..\script_component.hpp"

private _broadcastingUav = player getVariable [QGVAR(broadcastingUav), objNull];

if !(player getVariable [QEGVAR(client,eudConnected), false]) exitWith {
	if (!isNull _broadcastingUav) then {
		_broadcastingUav setVariable ["armatak_uav_mavlink_broadcasting", false, true];
		player setVariable [QGVAR(broadcastingUav), objNull];
	};
};

private _uav = getConnectedUAV player;
if (isNull _uav) then {
	_uav = _broadcastingUav;
};

if (isNull _uav || {!alive _uav}) exitWith {
	if (!isNull _broadcastingUav) then {
		_broadcastingUav setVariable ["armatak_uav_mavlink_broadcasting", false, true];
		player setVariable [QGVAR(broadcastingUav), objNull];
		systemChat "UAV broadcasting stopped";
		"armatak" callExtension ["log", [["info", "UAV broadcasting stopped because the UAV is no longer available"]]];
	};
};

if (_broadcastingUav isNotEqualTo _uav) then {
	if (!isNull _broadcastingUav) then {
		_broadcastingUav setVariable ["armatak_uav_mavlink_broadcasting", false, true];
	};
	player setVariable [QGVAR(broadcastingUav), _uav];
	_uav setVariable ["armatak_uav_mavlink_broadcasting", true, true];
	private _callsign = [_uav] call armatak_fnc_extract_marker_callsign;
	systemChat format ["Broadcasting UAV %1", _callsign];
	"armatak" callExtension ["log", [["info", format ["Broadcasting UAV %1 via MAVLink mock to %2", _callsign, player getVariable [QEGVAR(client,mavlink_address), ""]]]]];
};

_uav setVariable ["armatak_uav_mavlink_broadcasting", true, true];

private _mavlinkAddress = player getVariable [QEGVAR(client,mavlink_address), ""];
if (_mavlinkAddress isEqualTo "") exitWith {};

private _pos = [_uav] call EFUNC(client,extractClientPosition);
private _uuid = [_uav] call armatak_fnc_extract_uuid;
private _callsign = [_uav] call armatak_fnc_extract_marker_callsign;
private _videoUri = [_uav] call FUNC(resolveVideoUri);
private _dir = vectorDir _uav;
private _up = vectorUp _uav;
private _yaw = getDir _uav;
private _pitch = asin (((_dir select 2) max -1) min 1);
private _roll = asin (((_up select 0) max -1) min 1);
private _relAlt = ((getPosATL _uav) select 2) max 0;
private _uavType = if (_uav isKindOf "Plane") then {1} else {[2, 3] select (_uav isKindOf "Helicopter")};

private _gimbalRoll = 0;
private _gimbalPitch = _pitch;
private _gimbalYaw = _yaw;
private _hfov = _uav getVariable ["armatak_uas_fov", 60];
private _vfov = _uav getVariable ["armatak_uas_vfov", (_hfov * 0.5625)];
private _imageLat = _pos select 1;
private _imageLon = _pos select 2;
private _imageAlt = _pos select 3;
private _cameraData = [_uav, "turret"] call armatak_fnc_extract_uas_camera_data;
private _uavControl = UAVControl _uav;
private _controlledTurretPath = _uavControl param [1, []];
private _hasTurretCamera = ((_controlledTurretPath isEqualType []) && {_controlledTurretPath isNotEqualTo []}) || {(allTurrets _uav) isNotEqualTo []};

if (_cameraData isEqualType [] && {(count _cameraData) >= 6}) then {
	_gimbalYaw = _cameraData param [0, _yaw];
	_gimbalPitch = _cameraData param [1, _pitch];
	_hfov = _cameraData param [2, _hfov];
	_vfov = _uav getVariable ["armatak_uas_vfov", (_hfov * 0.5625)];

	private _spiGeo = _cameraData param [5, []];
	if (_spiGeo isEqualType [] && {(count _spiGeo) >= 3}) then {
		_imageLat = _spiGeo select 0;
		_imageLon = _spiGeo select 1;
		_imageAlt = _spiGeo select 2;
	};
};

private _systemPayload = [
	_mavlinkAddress,
	_uuid,
	_callsign,
	_uavType,
	_pos select 1,
	_pos select 2,
	_pos select 3,
	_relAlt,
	_pos select 5,
	_pos select 6,
	_roll,
	_pitch,
	_yaw,
	parseNumber isEngineOn _uav,
	_gimbalRoll,
	_gimbalPitch,
	_gimbalYaw,
	_videoUri,
	_hfov,
	_vfov,
	_imageLat,
	_imageLon,
	_imageAlt,
	parseNumber _hasTurretCamera
];

"armatak" callExtension ["uas:send_uas_system", [_systemPayload]];
