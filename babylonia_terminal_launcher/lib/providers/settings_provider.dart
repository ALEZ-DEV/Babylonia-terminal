import 'package:flutter/material.dart';

import './../models/settings.dart';

class SettingsProvider with ChangeNotifier {
  final Settings _settings = Settings();

  set setSelectedBackgroundType(BackgroundType selectedBackground) {
    _settings.selectedBackgroundType = selectedBackground;
    notifyListeners();
  }

  get getSelectedBackgroundType {
    return _settings.selectedBackgroundType;
  }
}
