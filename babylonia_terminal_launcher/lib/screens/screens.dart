import 'package:babylonia_terminal_launcher/screens/settings_screen.dart';
import 'package:flutter/material.dart';
import 'package:yaru/icons.dart';

import 'package:babylonia_terminal_launcher/screens/home_screen.dart';

class Screens {
  static final _screens = <Widget, (Widget, Widget, String)>{
    const HomeScreen(): (
      const Icon(YaruIcons.home_filled),
      const Icon(YaruIcons.home),
      'Home',
    ),
    const SettingsScreen(): (
      const Icon(YaruIcons.settings_filled),
      const Icon(YaruIcons.settings),
      'Settings',
    ),
  };

  static List<Widget> drawerItem(
          {required Function(int) onSelected, required int selectedIndex}) =>
      [
        for (int i = 0; i < Screens._screens.values.length; i++)
          ListTile(
            selected: i == selectedIndex,
            leading: i == selectedIndex
                ? Screens._screens.values.elementAt(i).$1
                : Screens._screens.values.elementAt(i).$2,
            title: Text(Screens._screens.values.elementAt(i).$3),
            onTap: () => onSelected(i),
          ),
      ];

  static Widget getCurrent(int index) => Screens._screens.keys.toList()[index];
}
