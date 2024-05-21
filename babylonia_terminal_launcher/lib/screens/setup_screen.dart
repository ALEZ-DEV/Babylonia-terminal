import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import './../messages/game_state.pb.dart';
import './../providers/game_state_provider.dart';
import './../providers/settings_provider.dart';
import './setups/welcome_screen.dart';
import './setups/proton_screen.dart';

class SetupScreen extends StatelessWidget {
  const SetupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final gameStateProvider = Provider.of<GameStateProvider>(context);
    final settingsProvider = Provider.of<SettingsProvider>(context);
    final Widget content;
    if (settingsProvider.firstTime) {
      content = const WelcomeScreen();
    } else {
      switch (gameStateProvider.gameState) {
        case States.ProtonNotInstalled:
          content = const ProtonScreen();
        case States.DXVKNotInstalled:
          content = const Text('DXVK');
        default:
          content = const Text('???');
      }
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text("Babylonia Terminal"),
        centerTitle: true,
      ),
      body: content,
    );
  }
}
