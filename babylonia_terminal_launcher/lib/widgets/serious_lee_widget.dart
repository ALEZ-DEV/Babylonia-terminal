import 'package:flutter/material.dart';

class SeriousLeeWidget extends StatelessWidget {
  const SeriousLeeWidget({super.key, required this.title});
  final String title;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          SizedBox(
            height: 300,
            child: Image.asset('assets/images/Lee6.png'),
          ),
          Center(
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Text(
                title,
                style: const TextStyle(
                  fontSize: 34,
                ),
              ),
            ),
          )
        ],
      ),
    );
  }
}
