import 'package:flutter/material.dart';

import './../widgets/serious_lee_widget.dart';
import './../widgets/simple_button.dart';

class SetupScreen extends StatelessWidget {
  const SetupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Babylonia Terminal"),
        centerTitle: true,
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const SeriousLeeWidget(title: 'Welcome to Babylonia Terminal!'),
            const Text(
              'We have to setup some things first',
              style: TextStyle(
                fontSize: 20,
              ),
            ),
            Padding(
              padding: const EdgeInsets.only(top: 15.0),
              child: SizedBox(
                width: 200,
                child: SimpleButton(
                  onPressed: () {},
                  child: const Text('Start'),
                ),
              ),
            )
          ],
        ),
      ),
    );
  }
}
