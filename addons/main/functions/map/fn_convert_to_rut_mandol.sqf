params["_latitude", "_longitude", "_altitude"];

_playerPosition = [_latitude, _longitude, _altitude];

_playerLatitude = _playerPosition select 0;
_playerLongitude = _playerPosition select 1;

_playerMaxLatitude = 32768;
_playerMaxLongitude = 32768;

_MapMaxLongitude = 35.285485;
_MapMinLongitude = 34.927617;

_MapMaxLatitude = 70.445404;
_MapMinLatitude = 70.016783;

_LongitudeDifference = _MapMaxLongitude - _MapMinLongitude;
_LatitudeDifference = _MapMaxLatitude - _MapMinLatitude;

_RealLongitude = (_playerLongitude / _playerMaxLongitude) * _LongitudeDifference + _MapMinLongitude;
_RealLatitude = (_playerLatitude / _playerMaxLatitude) * _LatitudeDifference + _MapMinLatitude;

[_RealLongitude, _RealLatitude, _playerPosition select 2]
