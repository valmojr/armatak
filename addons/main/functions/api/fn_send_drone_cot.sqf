// function name: armatak_fnc_send_drone_cot
// function author: Valmo
// function description: Gets the drone information for the CoT Router

params["_drone"];

[_drone] call armatak_fnc_send_uas_platform_cot;
[_drone] call armatak_fnc_send_uas_video_cot;
[_drone] call armatak_fnc_send_uas_sensor_cot;
