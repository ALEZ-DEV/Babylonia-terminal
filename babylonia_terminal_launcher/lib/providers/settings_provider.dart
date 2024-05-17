import 'package:flutter/material.dart';

import './../models/settings.dart';

class SettingsProvider with ChangeNotifier {
  late final Settings _settings;

  Future init() async {
    _settings = await Settings.create();
  }

  set setSelectedBackgroundType(BackgroundType selectedBackground) {
    _settings.selectedBackgroundType = selectedBackground;
    notifyListeners();
  }

  get getSelectedBackgroundType {
    return _settings.selectedBackgroundType;
  }
}
