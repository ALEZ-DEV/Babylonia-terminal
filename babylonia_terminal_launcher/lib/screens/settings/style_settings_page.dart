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
  late BackgroundType _radioValue;

  void setBackgroundValue(
    BuildContext context,
    SettingsProvider provider,
    BackgroundType? value,
  ) {
    if (value != null) {
      provider.setSelectedBackgroundType = value;
      setState(() {
        _radioValue = value;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<SettingsProvider>(context, listen: false);
    _radioValue = provider.getSelectedBackgroundType;

    return Center(
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(15.0),
            child: SizedBox(
              child: YaruSection(
                headline: const Text('Background video'),
                child: Column(
                  children: Settings.backgoundList
                      .map(
                        (b) => YaruRadioListTile(
                          value: b,
                          groupValue: _radioValue,
                          onChanged: (v) =>
                              setBackgroundValue(context, provider, v),
                          toggleable: true,
                          title: Text(
                            Settings.getStringNameOfBackgroundType(
                              b,
                            ),
                          ),
                        ),
                      )
                      .toList(),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
