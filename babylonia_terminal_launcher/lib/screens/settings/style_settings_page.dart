import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/yaru.dart';

import './../../providers/settings_provider.dart';
import './../../models/settings.dart';

class StyleSettingsPage extends StatefulWidget {
  const StyleSettingsPage({super.key});

  @override
  State<StyleSettingsPage> createState() => _StyleSettingsPageState();
}

class _StyleSettingsPageState extends State<StyleSettingsPage> {
  late int _radioValue;

  void setBackgroundValue(
    BuildContext context,
    SettingsProvider provider,
    int? value,
  ) {
    if (value != null) {
      provider.setSelectedBackgroundType = Settings.backgoundList[value];
      setState(() {
        _radioValue = value;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<SettingsProvider>(context, listen: false);
    _radioValue = Settings.backgoundList.indexWhere(
      (element) => element == provider.getSelectedBackgroundType,
    );

    return Center(
      child: YaruSection(
        headline: const Text('Background video'),
        child: Column(
          children: [
            for (int i = 0; i < Settings.backgoundList.length; i++)
              YaruRadioListTile(
                value: i,
                groupValue: _radioValue,
                onChanged: (v) => setBackgroundValue(context, provider, v),
                toggleable: true,
                title: Text(
                  Settings.getStringNameOfBackgroundType(
                    Settings.backgoundList[i],
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}
