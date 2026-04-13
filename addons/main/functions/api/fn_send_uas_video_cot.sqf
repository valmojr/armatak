// function name: armatak_fnc_send_uas_video_cot
// function author: Valmo / ArmaTAK contributors
// function description:
//   Sends a b-i-v CoT event that declares the RTSP video endpoint for a drone.
//   The ATAK UAS Tool picks this up and shows the drone in its UAS list with
//   the associated video feed available for playback.
//
//   The drone entity MUST have the variable "armatak_attribute_video_url" set
//   to a valid RTSP URL, e.g.:
//       _drone setVariable ["armatak_attribute_video_url", "rtsp://192.168.1.10:8554/live/drone1"];
//   or via the 3DEN attribute "Video URL (RTSP)" in the ARMA Team Awareness Kit
//   attribute category.
//
//   If the variable is absent or empty the function exits silently.
//
// Arguments:
//   0: _drone <OBJECT>  The drone object.
//
// Return value: none

params ["_drone"];

private _video_url = _drone getVariable ["armatak_attribute_video_url", ""];
if (_video_url == "") exitWith {};

private _uuid     = _drone call armatak_fnc_extract_uuid;
private _callsign = [_drone] call armatak_fnc_extract_marker_callsign;

private _payload = [_uuid, _callsign, _video_url];
"armatak" callExtension ["tcp_socket:cot:uas_video", [_payload]];
