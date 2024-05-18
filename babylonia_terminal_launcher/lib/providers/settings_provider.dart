import 'package:flutter/material.dart';

import './../models/settings.dart';
import './../models/config.dart';

class SettingsProvider with ChangeNotifier {
  late final Settings _settings;

  Future init() async {
    _settings = await Settings.create();
    await Config.update();
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
