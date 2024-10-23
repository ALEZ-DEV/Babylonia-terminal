import 'dart:async';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

import './config.dart';

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
  String? _launchOptions;

  Settings({required this.prefs, required String? launchOptions})
      : _launchOptions = launchOptions;

  static Future<Settings> create() async {
    final prefs = await SharedPreferences.getInstance();
    final launchOptions = await Config.getLaunchOptions();
    return Settings(
      prefs: prefs,
      launchOptions: launchOptions,
    );
  }

  final String _firstTimeKey = 'first_time';
  bool? get firstTime {
    return prefs.getBool(_firstTimeKey);
  }

  set firstTime(bool? value) {
    prefs.setBool(_firstTimeKey, value!);
  }

  final String _lastVersionKey = 'last_version';
  String? get lastVersion {
    return prefs.getString(_lastVersionKey);
  }

  set lastVersion(String? value) {
    prefs.setString(_lastVersionKey, value!);
  }

  String? get launchOptions {
    return _launchOptions;
  }

  set launchOptions(String? value) {
    Config.setLaunchOptions(value);
    _launchOptions = value;
  }

  BackgroundType? _backgroundType;
  final String _backgroundTypeKey = 'background_type';

  BackgroundType get selectedBackgroundType {
    final bt = prefs.getString(_backgroundTypeKey);
    if (bt == null) {
      _backgroundType = BackgroundType.disable;
      prefs.setString(
        _backgroundTypeKey,
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
      _backgroundTypeKey,
      getStringNameOfBackgroundType(selectedBackground),
    );
  }

  int? _backgroundId;
  String backgroundIdKey = 'background_id';

  int? get backgroundId {
    _backgroundId ??= prefs.getInt(backgroundIdKey);

    return _backgroundId;
  }

  set backgroundId(int? value) {
    _backgroundId = value;
    prefs.setInt(backgroundIdKey, value!);
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
