#include "..\script_component.hpp"

params [["_uav", objNull, [objNull]]];

private _defaultVideoUri = "rtsp://irontak.com:554/fpv";
private _activelyControlledUav = if (!isNull player) then {getConnectedUAV player} else {objNull};

private _normalize = {
	params ["_rawUrl"];

	private _url = trim _rawUrl;
	if (_url isEqualTo "") exitWith {""};

	if (_url find "://" >= 0) exitWith {_url};

	if (_url find "/" >= 0) exitWith {
		format ["rtsp://%1", _url]
	};

	format ["rtp://%1", _url]
};

if (!isNull _uav) then {
	private _objectVideoUrl = [_uav] call armatak_fnc_extract_marker_video_url;
	private _normalizedObjectVideoUrl = [_objectVideoUrl] call _normalize;
	if (_normalizedObjectVideoUrl isNotEqualTo "") exitWith {
		_normalizedObjectVideoUrl
	};

	private _activeSessionVideoUrl = player getVariable [QEGVAR(client,video_feed_url), ""];
	private _normalizedActiveSessionVideoUrl = [_activeSessionVideoUrl] call _normalize;
	if (_normalizedActiveSessionVideoUrl isNotEqualTo "") exitWith {
		_normalizedActiveSessionVideoUrl
	};

	_defaultVideoUri
};

private _sessionVideoUrl = player getVariable [QEGVAR(client,video_feed_url), ""];
private _normalizedSessionVideoUrl = [_sessionVideoUrl] call _normalize;
if (_normalizedSessionVideoUrl isNotEqualTo "") exitWith {
	_normalizedSessionVideoUrl
};

_defaultVideoUri
