params["_latitude", "_longitude", "_altitude"];

_playerPosition = [_latitude, _longitude, _altitude];

_playerLatitude = _playerPosition select 0;
_playerLongitude = _playerPosition select 1;

_playerMaxLongitude = 5120;
_playerMaxLatitude = 5120;

_MapMaxLatitude = -51.619725;
_MapMinLatitude = -51.664223;

_MapMaxLongitude = -58.394630;
_MapMinLongitude = -58.469580;

_LongitudeDifference = _MapMaxLongitude - _MapMinLongitude;
_LatitudeDifference = _MapMaxLatitude - _MapMinLatitude;

_RealLongitude = (_playerLongitude / _playerMaxLongitude) * _LongitudeDifference + _MapMinLongitude;
_RealLatitude = (_playerLatitude / _playerMaxLatitude) * _LatitudeDifference + _MapMinLatitude;

[_RealLongitude, _RealLatitude, _playerPosition select 2]
