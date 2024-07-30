import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

import './../models/settings.dart';
import './../models/config.dart';

class SettingsProvider with ChangeNotifier {
  late final Settings _settings;

  Future init() async {
    _settings = await Settings.create();
    if (kDebugMode) {
      _settings.firstTime = true;
      _settings.prefs.clear();
    }
    await Config.update();
  }

  bool get firstTime {
    final result = _settings.firstTime;
    if (result == null) {
      _settings.firstTime = true;
    }
    return _settings.firstTime!;
  }

  set firstTime(bool value) {
    _settings.firstTime = value;
    notifyListeners();
  }

  set setSelectedBackgroundType(BackgroundType selectedBackground) {
    _settings.selectedBackgroundType = selectedBackground;
    notifyListeners();
  }

  get getSelectedBackgroundType {
    return _settings.selectedBackgroundType;
  }

  int? get backgroundId {
    return _settings.backgroundId;
  }

  set backgroundId(int? value) {
    _settings.backgroundId = value;
  }
}
