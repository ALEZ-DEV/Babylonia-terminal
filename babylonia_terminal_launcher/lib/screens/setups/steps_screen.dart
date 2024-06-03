import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:provider/provider.dart';
import 'package:yaru/yaru.dart';

import './../../messages/game_state.pb.dart';
import './../../providers/providers.dart';
import './../../widgets/selectable_yaru_expansion_panel.dart';
import './../../widgets/simple_button.dart';
import './../../widgets/steps/proton_steps_widget.dart';
import './../../widgets/steps/dxvk_steps_widget.dart';
import './../../widgets/steps/fonts_steps_widget.dart';
import './../../widgets/steps/dependencies_steps_widget.dart';

class StepsScreen extends StatelessWidget {
  StepsScreen({super.key});
  final SectionController controller = SectionController(selectedItem: 0);

  @override
  Widget build(BuildContext context) {
    final gameStateProvider = Provider.of<GameStateProvider>(context);
    switch (gameStateProvider.gameState) {
      case States.ProtonNotInstalled:
        controller.updateSection(0);
        break;
      case States.DXVKNotInstalled:
        controller.updateSection(1);
        break;
      case States.FontNotInstalled:
        controller.updateSection(2);
        break;
      case States.DependecieNotInstalled:
        controller.updateSection(3);
        break;
      default:
        controller.updateSection(null);
        break;
    }

    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          ConstrainedBox(
            constraints: const BoxConstraints(
              maxHeight: 600,
              maxWidth: 750,
            ),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 25.0),
              child: SelectableYaruExpansionPanel(
                controller: controller,
                headers: const [
                  Text("Proton"),
                  Text("DXVK"),
                  Text("Fonts"),
                  Text("Dependencies"),
                ],
                children: const [
                  ProtonSteps(),
                  DXVKSteps(),
                  FontsSteps(),
                  DependenciesSteps(),
                ],
              ),
            ),
          ),
          Padding(
            padding: const EdgeInsets.only(top: 12.0),
            child: SimpleButton(
              onPressed: !gameStateProvider.needToSetup()
                  ? () => gameStateProvider.updateSetup()
                  : null,
              child: const Text('next'),
            ),
          ),
        ],
      ),
    );
  }
}
