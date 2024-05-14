import 'package:flutter/material.dart';

import './../widgets/background.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        const Background(),
        SizedBox(
          child: Align(
            alignment: Alignment.bottomRight,
            child: Padding(
              padding: const EdgeInsets.only(right: 50, bottom: 50),
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.blue[500],
                ),
                onPressed: () {},
                child: const SizedBox(
                  width: 300,
                  height: 25,
                  child: Center(
                    child: Text("Download"),
                  ),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
