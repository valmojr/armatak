params ["_drone", ["_cameraData", []]];

if (isNull _drone) exitWith {};

if ((_cameraData isEqualType []) && {(count _cameraData) >= 6}) then {
    _drone setVariable ["armatak_uas_camera_data_override", _cameraData + [serverTime], false];
} else {
    _drone setVariable ["armatak_uas_camera_data_override", nil, false];
};
