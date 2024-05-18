import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../widgets/background_widget.dart';
import '../widgets/serious_lee_widget.dart';
import '../widgets/simple_button.dart';
import './../providers/providers.dart';
import './../models/settings.dart';
import './../models/background.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        const ShowBackground(),
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
                        child: SimpleButton(
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

class ShowBackground extends StatefulWidget {
  const ShowBackground({super.key});

  @override
  State<ShowBackground> createState() => _ShowBackgroundState();
}

class _ShowBackgroundState extends State<ShowBackground> {
  bool isLoading = false;
  bool hadLoaded = false;
  late final Background _background;

  @override
  void didChangeDependencies() async {
    if (!hadLoaded) {
      isLoading = true;

      _background =
          await Background.get(Provider.of<SettingsProvider>(context));

      setState(() {
        isLoading = false;
        hadLoaded = true;
      });
    }

    super.didChangeDependencies();
  }

  @override
  Widget build(BuildContext context) {
    if (Provider.of<SettingsProvider>(context).getSelectedBackgroundType !=
        BackgroundType.disable) {
      if (isLoading) {
        return const SeriousLeeWidget(
          title: 'Babylonia Terminal',
        );
      } else {
        return BackgroundWidget(
          background: _background,
        );
      }
    } else {
      return const SeriousLeeWidget(
        title: 'Babylonia Terminal',
      );
    }
  }
}
