import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import './../../providers/settings_provider.dart';
import './../../widgets/serious_lee_widget.dart';
import './../../widgets/simple_button.dart';

class WelcomeScreen extends StatelessWidget {
  const WelcomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const SeriousLeeWidget(title: 'Welcome to Babylonia Terminal!'),
          const Text(
            'We have to setup some things first',
            style: TextStyle(
              fontSize: 20,
            ),
          ),
          Padding(
            padding: const EdgeInsets.only(top: 15.0),
            child: SizedBox(
              width: 200,
              child: SimpleButton(
                onPressed: () =>
                    Provider.of<SettingsProvider>(context, listen: false)
                        .firstTime = false,
                child: const Text('Start'),
              ),
            ),
          )
        ],
      ),
    );
  }
}
