import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/yaru.dart';

import './../../providers/settings_provider.dart';

class GeneralSettingsPage extends StatefulWidget {
  const GeneralSettingsPage({super.key});

  @override
  State<GeneralSettingsPage> createState() => _GeneralSettingsPageState();
}

class _GeneralSettingsPageState extends State<GeneralSettingsPage> {
  final textFieldController = TextEditingController();
  String? _launchOptionsError;

  bool isLaunchOptionsValid(String launchOptions) {
    return RegExp(r'^.*%command%.*$').hasMatch(launchOptions);
  }

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<SettingsProvider>(context, listen: false);
    final launchOptions = provider.launchOptions;

    return Center(
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(15.0),
            child: SizedBox(
              child: YaruSection(
                headline: const Text('Launch options'),
                child: TextFormField(
                  initialValue: launchOptions,
                  decoration: InputDecoration(
                    labelText:
                        'Your custom launch options, E.G. : mangohud %command%',
                    errorText: _launchOptionsError,
                  ),
                  validator: (value) => _launchOptionsError,
                  onChanged: (String text) {
                    if (text.isEmpty) {
                      setState(() {
                        _launchOptionsError = null;
                      });
                      provider.launchOptions = null;
                    } else if (!isLaunchOptionsValid(text)) {
                      setState(() {
                        _launchOptionsError =
                            'You need to put \'%command%\' in your command';
                      });
                    } else {
                      setState(() {
                        _launchOptionsError = null;
                      });
                      provider.launchOptions = text;
                    }
                  },
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
