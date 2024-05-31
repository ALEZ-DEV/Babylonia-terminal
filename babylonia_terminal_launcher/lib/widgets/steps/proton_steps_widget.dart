import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/widgets.dart';

import './../../models/github.dart';
import './../../models/proton.dart';
import './../../widgets/simple_button.dart';
import './../../widgets/gtk_spinner_widget.dart';
import './../../providers/providers.dart';

class ProtonSteps extends StatefulWidget {
  const ProtonSteps({super.key});

  @override
  State<ProtonSteps> createState() => _ProtonStepsState();
}

class _ProtonStepsState extends State<ProtonSteps> {
  final Proton proton = Proton();

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: ChangeNotifierProvider(
        create: (_) => proton,
        child: Builder(
          builder: (context) {
            switch (Provider.of<Proton>(context).protonState) {
              case ProtonInstallationState.idle:
                return const InstallProton();
              case ProtonInstallationState.downloading:
                return const ProtonDownloadProgress();
              case ProtonInstallationState.decompressing:
                return const Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Padding(
                      padding: EdgeInsets.all(8.0),
                      child: GtkSpinner(),
                    ),
                    Text('decompressing...'),
                  ],
                );
            }
          },
        ),
      ),
    );
  }
}

class InstallProton extends StatefulWidget {
  const InstallProton({super.key});

  @override
  State<InstallProton> createState() => _InstallProtonState();
}

class _InstallProtonState extends State<InstallProton> {
  bool hasLoaded = false;
  bool isLoading = false;
  late List<String> protonVersions;
  String? selectedValue;
  bool canInstall = true;

  @override
  void didChangeDependencies() async {
    if (!hasLoaded) {
      isLoading = true;
      protonVersions = await Github.getProtonVersions();
      setState(() {
        isLoading = false;
        hasLoaded = true;
      });
    }

    super.didChangeDependencies();
  }

  @override
  Widget build(BuildContext context) {
    if (hasLoaded && selectedValue == null) {
      selectedValue = protonVersions.first;
    }

    return isLoading
        ? const Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Padding(
                padding: EdgeInsets.all(8.0),
                child: GtkSpinner(),
              ),
              Text('Fetching versions...'),
            ],
          )
        : Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              YaruPopupMenuButton(
                initialValue: selectedValue,
                itemBuilder: (_) => protonVersions
                    .map(
                      (e) => PopupMenuItem(
                        value: e,
                        child: Text(e),
                      ),
                    )
                    .toList(),
                onSelected: (v) => setState(() {
                  selectedValue = v;
                }),
                child: Text(selectedValue!),
              ),
              Padding(
                padding: const EdgeInsets.only(left: 8.0),
                child: SimpleButton(
                  onPressed: canInstall
                      ? () {
                          Provider.of<Proton>(context, listen: false)
                              .startInstallation(
                                  Provider.of<GameStateProvider>(context,
                                      listen: false),
                                  selectedValue!);
                          setState(() {
                            canInstall = false;
                          });
                        }
                      : null,
                  child: const Text("Install"),
                ),
              ),
            ],
          );
  }
}

class ProtonDownloadProgress extends StatelessWidget {
  const ProtonDownloadProgress({super.key});

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<Proton>(context);
    final pourcent =
        (provider.currentProgress.toInt() / provider.maxProgress.toInt()) * 100;

    return Column(
      children: [
        Text("Downloaded: ${pourcent.toStringAsFixed(2)}%"),
        YaruLinearProgressIndicator(
          value: pourcent / 100,
        ),
      ],
    );
  }
}
