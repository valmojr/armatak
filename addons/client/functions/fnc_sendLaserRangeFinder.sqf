#include "..\script_component.hpp"

params ["_unit"];

private _lrfEnabled = player getVariable [QGVAR(lrfEnabled), false];

private _uid = format ["%1.LRF", _unit call armatak_fnc_extract_uuid];
private _laserTarget = laserTarget _unit;

if (!isNull _laserTarget) exitWith {
	private _originASL = getPosASL _unit;
	private _targetASL = getPosASL _laserTarget;
	private _delta = _targetASL vectorDiff _originASL;

	private _dx = _delta select 0;
	private _dy = _delta select 1;
	private _dz = _delta select 2;
	private _horizontalDistance = sqrt ((_dx * _dx) + (_dy * _dy));
	private _slantDistance = (_originASL vectorDistance _targetASL) max 1;
	private _azimuth = (((_dx atan2 _dy) + 360) mod 360);
	private _elevation = _dz atan2 (_horizontalDistance max 0.001);
	private _lastTargetASL = player getVariable [QGVAR(lrfLastTargetASL), []];
	private _lastSentAt = player getVariable [QGVAR(lrfLastSentAt), -1000];
	private _targetMoved = _lastTargetASL isEqualTo [] || {(_lastTargetASL vectorDistance _targetASL) > 5};
	private _sendCooldownElapsed = (time - _lastSentAt) >= 2.5;

	player setVariable [QGVAR(lrfWasActive), true];
	player setVariable [QGVAR(lrfLostAt), -1];
	player setVariable [QGVAR(lrfClearSent), false];

	if (_lrfEnabled && {_targetMoved} && {_sendCooldownElapsed}) then {
		"armatak" callExtension ["udp_socket:send_lrf", [[_uid, _slantDistance, _azimuth, _elevation]]];
		player setVariable [QGVAR(lrfLastTargetASL), _targetASL];
		player setVariable [QGVAR(lrfLastSentAt), time];
	};
};

if !(player getVariable [QGVAR(lrfWasActive), false]) exitWith {};

private _lostAt = player getVariable [QGVAR(lrfLostAt), -1];
if (_lostAt < 0) then {
	player setVariable [QGVAR(lrfLostAt), time];
};

private _clearSent = player getVariable [QGVAR(lrfClearSent), false];
if (_lrfEnabled && {!_clearSent} && {(time - (player getVariable [QGVAR(lrfLostAt), time])) >= 6}) then {
	"armatak" callExtension ["udp_socket:clear_lrf", [_uid]];
	player setVariable [QGVAR(lrfWasActive), false];
	player setVariable [QGVAR(lrfClearSent), true];
	player setVariable [QGVAR(lrfLastTargetASL), []];
	player setVariable [QGVAR(lrfLastSentAt), -1000];
};
