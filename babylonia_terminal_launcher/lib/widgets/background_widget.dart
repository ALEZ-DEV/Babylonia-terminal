import 'package:flutter/material.dart';
import 'package:media_kit/media_kit.dart';
import 'package:media_kit_video/media_kit_video.dart';
import 'package:provider/provider.dart';

import './../providers/settings_provider.dart';
import './../models/settings.dart';
import './../models/background.dart';

class BackgroundWidget extends StatefulWidget {
  const BackgroundWidget({super.key, required this.background});

  final Background background;

  @override
  State<BackgroundWidget> createState() => _BackgroundWidgetState();
}

class _BackgroundWidgetState extends State<BackgroundWidget> {
  late final player = Player(
    configuration: const PlayerConfiguration(),
  );
  late final controller = VideoController(player);

  @override
  void initState() {
    super.initState();
    player.open(
      Media(
        widget.background.path,
      ),
    );
    player.setPlaylistMode(PlaylistMode.loop);
  }

  @override
  void dispose() {
    player.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<SettingsProvider>(context);

    player.play();
    return Video(
      controller: controller,
      controls: NoVideoControls,
      wakelock: false,
      fit: Settings.getBoxFitFromBackgroundType(
        provider.getSelectedBackgroundType,
      ),
    );
  }
}
