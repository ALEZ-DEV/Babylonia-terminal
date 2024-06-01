import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:yaru/widgets.dart';

import './../../models/github.dart';
import './../../models/dxvk.dart';
import './../../widgets/simple_button.dart';
import './../../widgets/gtk_spinner_widget.dart';
import './../../providers/providers.dart';

class DXVKSteps extends StatefulWidget {
  const DXVKSteps({super.key});

  @override
  State<DXVKSteps> createState() => _DXVKStepsState();
}

class _DXVKStepsState extends State<DXVKSteps> {
  final DXVK proton = DXVK();

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: ChangeNotifierProvider(
        create: (_) => proton,
        child: Builder(
          builder: (context) {
            switch (Provider.of<DXVK>(context).dxvkState) {
              case DXVKInstallationState.idle:
                return const InstallDXVK();
              case DXVKInstallationState.downloading:
                return const DXVKDownloadProgress();
              case DXVKInstallationState.decompressing:
                return const Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Padding(
                      padding: EdgeInsets.all(8.0),
                      child: GtkSpinner(),
                    ),
                    Text('Installing...'),
                  ],
                );
            }
          },
        ),
      ),
    );
  }
}

class InstallDXVK extends StatefulWidget {
  const InstallDXVK({super.key});

  @override
  State<InstallDXVK> createState() => _InstallDXVKState();
}

class _InstallDXVKState extends State<InstallDXVK> {
  bool hasLoaded = false;
  bool isLoading = false;
  late List<String> protonVersions;
  String? selectedValue;
  bool canInstall = true;

  @override
  void didChangeDependencies() async {
    if (!hasLoaded) {
      isLoading = true;
      protonVersions = await Github.getDXVKVersions();
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
                          Provider.of<DXVK>(context, listen: false)
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

class DXVKDownloadProgress extends StatelessWidget {
  const DXVKDownloadProgress({super.key});

  @override
  Widget build(BuildContext context) {
    final provider = Provider.of<DXVK>(context);
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
