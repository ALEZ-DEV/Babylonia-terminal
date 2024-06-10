import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/yaru.dart';

import './../../models/game.dart';
import './../simple_button.dart';
import './../gtk_spinner_widget.dart';
import './../../providers/providers.dart';

class GameSteps extends StatelessWidget {
  const GameSteps({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(50.0),
      child: Builder(builder: (context) {
        switch (Provider.of<Game>(context).gameInstallationState) {
          case GameInstallationState.idle:
            return const _InstallGame();
          case GameInstallationState.checkingFile:
            return const _CheckingFileGame();
          case GameInstallationState.downloading:
            return const _DownloadingGame();
          case GameInstallationState.patching:
            return const _PatchingGame();
        }
      }),
    );
  }
}

class _InstallGame extends StatelessWidget {
  const _InstallGame({super.key});

  @override
  Widget build(BuildContext context) {
    final gameStateProvider = Provider.of<GameStateProvider>(context);

    return Row(
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
                  onPressed: () async {
                    Provider.of<Game>(context, listen: false).startInstallation(
                      Provider.of<GameStateProvider>(context, listen: false),
                      gameStateProvider.isGameUpdating(),
                    );
                  },
                  child: gameStateProvider.isGameUpdating()
                      ? const Center(
                          child: Text("Update"),
                        )
                      : const Center(
                          child: Text("Download"),
                        ),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class _CheckingFileGame extends StatelessWidget {
  const _CheckingFileGame({super.key});

  @override
  Widget build(BuildContext context) {
    return const Padding(
      padding: EdgeInsets.only(bottom: 40.0),
      child: Align(
        alignment: Alignment.bottomCenter,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            GtkSpinner(),
            Text("Checking files... (This can take a while)"),
          ],
        ),
      ),
    );
  }
}

class _DownloadingGame extends StatelessWidget {
  const _DownloadingGame({super.key});

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<Game>(context);
    final pourcent =
        (provider.currentProgress.toInt() / provider.maxProgress.toInt()) * 100;
    final currentGb = provider.currentProgress.toInt() / 1024 / 1024 / 1024;
    final maxGb = provider.maxProgress.toInt() / 1024 / 1024 / 1024;

    return Padding(
      padding: const EdgeInsets.only(bottom: 40.0),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 8.0),
            child: Text(
              "${currentGb.toStringAsFixed(2)} / ${maxGb.toStringAsFixed(2)} Gb (${pourcent.toStringAsFixed(2)}%)",
            ),
          ),
          YaruLinearProgressIndicator(
            value: pourcent / 100,
          ),
        ],
      ),
    );
  }
}

class _PatchingGame extends StatelessWidget {
  const _PatchingGame({super.key});

  @override
  Widget build(BuildContext context) {
    return const Padding(
      padding: EdgeInsets.only(bottom: 40.0),
      child: Align(
        alignment: Alignment.bottomCenter,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            GtkSpinner(),
            Text("Patching game"),
          ],
        ),
      ),
    );
  }
}
