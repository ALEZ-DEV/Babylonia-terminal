import 'package:babylonia_terminal_launcher/widgets/gtk_spinner_widget.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import './../../models/dependencies.dart';
import './../../providers/providers.dart';
import './../../widgets/simple_button.dart';

class DependenciesSteps extends StatelessWidget {
  const DependenciesSteps({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: ChangeNotifierProvider(
        create: (context) => Dependencies(),
        child: Builder(
          builder: (context) {
            switch (Provider.of<Dependencies>(context).dependeciesState) {
              case DependenciesInstallationState.idle:
                return const InstallDependecies();
              case DependenciesInstallationState.installing:
                return const DependeciesInstallationProgress();
            }
          },
        ),
      ),
    );
  }
}

class InstallDependecies extends StatefulWidget {
  const InstallDependecies({super.key});

  @override
  State<InstallDependecies> createState() => _InstallDependeciesState();
}

class _InstallDependeciesState extends State<InstallDependecies> {
  bool canInstall = true;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 8.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          SimpleButton(
            onPressed: canInstall
                ? () {
                    Provider.of<Dependencies>(context, listen: false)
                        .startInstallation(
                      Provider.of<GameStateProvider>(context, listen: false),
                    );
                    setState(() {
                      canInstall = false;
                    });
                  }
                : null,
            child: const Text("Install"),
          ),
          if (!canInstall) const GtkSpinner(),
        ],
      ),
    );
  }
}

class DependeciesInstallationProgress extends StatelessWidget {
  const DependeciesInstallationProgress({super.key});

  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        GtkSpinner(),
        Text(
          'Installing dependecies (this can take a while, so go grab a coffee)',
        ),
      ],
    );
  }
}
