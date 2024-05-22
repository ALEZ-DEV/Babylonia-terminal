import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import './../providers/settings_provider.dart';
import './setups/welcome_screen.dart';
import 'setups/steps_screen.dart';

class SetupScreen extends StatelessWidget {
  const SetupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final settingsProvider = Provider.of<SettingsProvider>(context);

    return Scaffold(
      appBar: AppBar(
        title: const Text("Babylonia Terminal"),
        centerTitle: true,
      ),
      body: settingsProvider.firstTime ? const WelcomeScreen() : StepsScreen(),
    );
  }
}
