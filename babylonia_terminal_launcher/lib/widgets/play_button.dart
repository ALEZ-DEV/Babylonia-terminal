import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import './simple_button.dart';
import './../models/game.dart';

class PlayButton extends StatelessWidget {
  const PlayButton({super.key});

  @override
  Widget build(BuildContext context) {
    final game = Provider.of<Game>(context);

    return Padding(
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
                  child: SimpleButton(
                    onPressed: game.gameRunState == GameRunState.idle
                        ? () {
                            Provider.of<Game>(context, listen: false)
                                .startGame();
                          }
                        : null,
                    child: Center(
                      child: game.gameRunState == GameRunState.idle
                          ? const Text("Play")
                          : const Text("Running..."),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
