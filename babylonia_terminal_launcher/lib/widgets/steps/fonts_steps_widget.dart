import 'package:babylonia_terminal_launcher/widgets/gtk_spinner_widget.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/yaru.dart';

import './../../models/fonts.dart';
import './../../providers/providers.dart';
import './../../widgets/simple_button.dart';

class FontsSteps extends StatelessWidget {
  const FontsSteps({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: ChangeNotifierProvider(
        create: (context) => Fonts(),
        child: Builder(
          builder: (context) {
            switch (Provider.of<Fonts>(context).fontsState) {
              case FontsInstallationState.idle:
                return const InstallFonts();
              case FontsInstallationState.installing:
                return const FontsInstallationProgress();
            }
          },
        ),
      ),
    );
  }
}

class InstallFonts extends StatefulWidget {
  const InstallFonts({super.key});

  @override
  State<InstallFonts> createState() => _InstallFontsState();
}

class _InstallFontsState extends State<InstallFonts> {
  bool canInstall = true;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 8.0),
      child: SimpleButton(
        onPressed: canInstall
            ? () {
                Provider.of<Fonts>(context, listen: false).startInstallation(
                  Provider.of<GameStateProvider>(context, listen: false),
                );
                setState(() {
                  canInstall = false;
                });
              }
            : null,
        child: const Text("Install"),
      ),
    );
  }
}

class FontsInstallationProgress extends StatelessWidget {
  const FontsInstallationProgress({super.key});

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<Fonts>(context);
    final pourcent =
        (provider.currentProgress.toInt() / provider.maxProgress.toInt()) * 100;

    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const GtkSpinner(),
              Text(
                  "${provider.currentProgress} / ${provider.maxProgress} fonts installed"),
            ],
          ),
        ),
        YaruLinearProgressIndicator(
          value: pourcent / 100,
        ),
      ],
    );
  }
}
