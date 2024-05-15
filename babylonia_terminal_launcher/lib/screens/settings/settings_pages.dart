import 'package:flutter/material.dart';
import 'package:yaru/yaru.dart';

import './general_settings_page.dart';
import './game_settings_page.dart';
import './style_settings_page.dart';

class SettingsPage {
  static final _pages = <Widget, (Widget, String)>{
    const GeneralSettingsPage(): (
      const Icon(YaruIcons.gears),
      'General',
    ),
    const GameSettingsPage(): (
      const Icon(YaruIcons.game_controller_filled),
      'Game',
    ),
    const StyleSettingsPage(): (
      const Icon(YaruIcons.desktop_appearance_filled),
      'Style',
    ),
  };

  static List<Widget> tabItems() => [
        for (int i = 0; i < SettingsPage._pages.values.length; i++)
          YaruTab(
            icon: SettingsPage._pages.values.elementAt(i).$1,
            label: SettingsPage._pages.values.elementAt(i).$2,
          ),
      ];

  static List<Widget> getPages() => SettingsPage._pages.keys.toList();
}
