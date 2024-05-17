import 'dart:async';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

enum BackgroundType {
  contain,
  fill,
  cover,
  fitHeight,
  fitWidth,
  disable,
}

class Settings {
  final SharedPreferences prefs;

  Settings({required this.prefs});

  static Future<Settings> create() async {
    final prefs = await SharedPreferences.getInstance();
    return Settings(prefs: prefs);
  }

  BackgroundType? _backgroundType;
  String backgroundTypeKey = 'background_type';

  BackgroundType get selectedBackgroundType {
    final bt = prefs.getString(backgroundTypeKey);
    if (bt == null) {
      _backgroundType = BackgroundType.disable;
      prefs.setString(
        backgroundTypeKey,
        getStringNameOfBackgroundType(_backgroundType!),
      );
    } else {
      _backgroundType ??= getBackgroundTypeFromString(bt);
    }

    return _backgroundType!;
  }

  set selectedBackgroundType(BackgroundType selectedBackground) {
    _backgroundType = selectedBackground;
    prefs.setString(
      backgroundTypeKey,
      getStringNameOfBackgroundType(selectedBackground),
    );
  }

  static final List<BackgroundType> backgoundList = [
    BackgroundType.contain,
    BackgroundType.fill,
    BackgroundType.cover,
    BackgroundType.fitHeight,
    BackgroundType.fitWidth,
    BackgroundType.disable,
  ];

  static String getStringNameOfBackgroundType(BackgroundType backgroundType) {
    switch (backgroundType) {
      case BackgroundType.contain:
        return 'contain';
      case BackgroundType.fill:
        return 'fill';
      case BackgroundType.cover:
        return 'cover';
      case BackgroundType.fitHeight:
        return 'fitHeight';
      case BackgroundType.fitWidth:
        return 'fitWidth';
      case BackgroundType.disable:
        return 'disable';
    }
  }

  static BackgroundType getBackgroundTypeFromString(String background) {
    switch (background) {
      case 'contain':
        return BackgroundType.contain;
      case 'fill':
        return BackgroundType.fill;
      case 'cover':
        return BackgroundType.cover;
      case 'fitHeight':
        return BackgroundType.fitHeight;
      case 'fitWidth':
        return BackgroundType.fitWidth;
      case 'disable':
        return BackgroundType.disable;
      default:
        throw FormatException('Can\'t convert String to Enum BackgroundType!');
    }
  }

  static BoxFit getBoxFitFromBackgroundType(BackgroundType backgroundType) {
    switch (backgroundType) {
      case BackgroundType.contain:
        return BoxFit.contain;
      case BackgroundType.fill:
        return BoxFit.fill;
      case BackgroundType.cover:
        return BoxFit.cover;
      case BackgroundType.fitHeight:
        return BoxFit.fitHeight;
      case BackgroundType.fitWidth:
        return BoxFit.fitWidth;
      case BackgroundType.disable:
        throw FormatException(
            'Can\'t convert BackgroundType to widget BoxFit!');
    }
  }
}
