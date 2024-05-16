import 'package:flutter/material.dart';
import 'package:media_kit/media_kit.dart';
import 'package:media_kit_video/media_kit_video.dart';
import 'package:provider/provider.dart';

import './../providers/settings_provider.dart';
import './../models/settings.dart';

class Background extends StatefulWidget {
  const Background({super.key});

  @override
  State<Background> createState() => _BackgroundState();
}

class _BackgroundState extends State<Background> {
  late final player = Player(
    configuration: PlayerConfiguration(),
  );
  late final controller = VideoController(player);

  @override
  void initState() {
    super.initState();
    player.open(
      Media(
        '/home/alez/Downloads/1l5uvj9eqpjjcazt4p-1715219648481.mp4',
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
