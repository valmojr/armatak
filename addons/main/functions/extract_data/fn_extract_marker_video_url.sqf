// function name: armatak_fnc_extract_marker_video_url
// function author: Codex
// function description: Gets the marker video URL configured in 3DEN for a vehicle

params ["_unit"];

private _videoUrl = _unit getVariable ["armatak_attribute_marker_video_url", ""];

if (isNil "_videoUrl") exitWith {
	""
};

_videoUrl
