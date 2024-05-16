import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import './../widgets/background.dart';
import './../providers/providers.dart';
import './../models/settings.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        if (Provider.of<SettingsProvider>(context).getSelectedBackgroundType !=
            BackgroundType.disable)
          const Background()
        else
          Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                SizedBox(
                  height: 300,
                  child: Image.asset('assets/images/Lee6.png'),
                ),
                const Center(
                  child: Padding(
                    padding: EdgeInsets.all(8.0),
                    child: Text(
                      'Babylonia Terminal',
                      style: TextStyle(
                        fontSize: 34,
                      ),
                    ),
                  ),
                )
              ],
            ),
          ),
        SizedBox(
          child: Padding(
            padding: const EdgeInsets.all(50.0),
            child: Row(
              children: [
                const Expanded(
                  child: SizedBox(),
                ),
                const Expanded(
                  child: SizedBox(),
                ),
                Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      ConstrainedBox(
                        constraints: const BoxConstraints(
                          maxWidth: 600,
                          maxHeight: 50,
                        ),
                        child: ElevatedButton(
                          style: ElevatedButton.styleFrom(
                            backgroundColor: Colors.blue[500],
                          ),
                          onPressed: () async {
                            Provider.of<GameStateProvider>(context,
                                    listen: false)
                                .updateGameState();
                          },
                          child: const Center(
                            child: Text("Download"),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
