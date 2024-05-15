import 'package:flutter/material.dart';

enum BackgroundType {
  contain,
  fill,
  cover,
  fitHeight,
  fitWidth,
  disable,
}

class Settings {
  BackgroundType _backgroundType = BackgroundType.cover;

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

  BackgroundType get selectedBackgroundType {
    return _backgroundType;
  }

  set selectedBackgroundType(BackgroundType selectedBackground) {
    _backgroundType = selectedBackground;
  }
}
