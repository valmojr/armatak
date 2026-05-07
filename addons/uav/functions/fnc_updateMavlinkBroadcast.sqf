#include "..\script_component.hpp"

private _broadcastingUav = player getVariable [QGVAR(broadcastingUav), objNull];

if !(player getVariable [QEGVAR(client,eudConnected), false]) exitWith {
	if (!isNull _broadcastingUav) then {
		_broadcastingUav setVariable ["armatak_uav_mavlink_broadcasting", false, true];
		player setVariable [QGVAR(broadcastingUav), objNull];
	};
};

private _uav = getConnectedUAV player;

if (isNull _uav) exitWith {
	if (!isNull _broadcastingUav) then {
		_broadcastingUav setVariable ["armatak_uav_mavlink_broadcasting", false, true];
		player setVariable [QGVAR(broadcastingUav), objNull];
		systemChat "UAV broadcasting stopped";
		"armatak" callExtension ["log", [["info", "UAV broadcasting stopped because player is no longer connected to a UAV"]]];
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

private _laser = laserTarget _uav;
if (!isNull _laser) then {
	private _originASL = getPosASL _uav;
	private _targetWorld = getPosWorld _laser;
	private _targetAslZ = (getPosASL _laser) select 2;
	private _spiASL = [_targetWorld select 0, _targetWorld select 1, _targetAslZ];

	private _los = _spiASL vectorDiff _originASL;
	private _losMag = vectorMagnitude _los;

	if (_losMag > 0) then {
		_los = _los vectorMultiply (1 / _losMag);

		private _dirX = _los select 0;
		private _dirY = _los select 1;
		private _dirZ = _los select 2;
		private _horizontalMag = sqrt ((_dirX * _dirX) + (_dirY * _dirY));

		private _camYaw = ((_dirX atan2 _dirY) + 360) mod 360;
		private _camPitch = _dirZ atan2 (_horizontalMag max 0.001);
		_gimbalPitch = _camPitch;
		_gimbalYaw = _camYaw;

		private _imageGeo = [_targetWorld select 0, _targetWorld select 1, _targetAslZ] call EFUNC(client,convertClientLocation);
		_imageLat = _imageGeo select 0;
		_imageLon = _imageGeo select 1;
		_imageAlt = _imageGeo select 2;
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
	_imageAlt
];

"armatak" callExtension ["uas:send_uas_system", [_systemPayload]];
