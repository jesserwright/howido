import 'dart:async';
import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:flutter_feather_icons/flutter_feather_icons.dart';
import 'package:path/path.dart' show join;
import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  // Ensure that plugin services are initialized so that `availableCameras()`
  // can be called before `runApp()`
  WidgetsFlutterBinding.ensureInitialized();

  // Obtain a list of the available cameras on the device.
  final cameras = await availableCameras();

  // Get a specific camera from the list of available cameras.
  final firstCamera = cameras.first;

  runApp(
    MaterialApp(
      theme: ThemeData.dark(),
      home: TakePictureScreen(
        // Pass the appropriate camera to the TakePictureScreen widget.
        camera: firstCamera,
      ),
    ),
  );
}

// A screen that allows users to take a picture using a given camera.
class TakePictureScreen extends StatefulWidget {
  final CameraDescription camera;

  const TakePictureScreen({
    Key key,
    @required this.camera,
  }) : super(key: key);

  @override
  TakePictureScreenState createState() => TakePictureScreenState();
}

class TakePictureScreenState extends State<TakePictureScreen> {
  CameraController _controller;
  Future<void> _initializeControllerFuture;

  @override
  void initState() {
    super.initState();
    // To display the current output from the Camera,
    // create a CameraController.
    _controller = CameraController(
      // Get a specific camera from the list of available cameras.
      widget.camera,
      // Define the resolution to use.
      ResolutionPreset.max,
      enableAudio: false,
    );

    // Next, initialize the controller. This returns a Future.
    _initializeControllerFuture = _controller.initialize();
  }

  @override
  void dispose() {
    // Dispose of the controller when the widget is disposed.
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Take a picture')),
      // Wait until the controller is initialized before displaying the
      // camera preview. Use a FutureBuilder to display a loading spinner
      // until the controller has finished initializing.
      body: FutureBuilder<void>(
        future: _initializeControllerFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.done) {
            // If the Future is complete, display the preview.
            return CameraPreview(_controller);
          } else {
            // Otherwise, display a loading indicator.
            return Center(child: CircularProgressIndicator());
          }
        },
      ),
      floatingActionButton: FloatingActionButton(
        child: Icon(Icons.camera_alt),
        // Provide an onPressed callback.
        onPressed: () async {
          // Take the Picture in a try / catch block. If anything goes wrong,
          // catch the error.
          try {
            // Ensure that the camera is initialized.
            await _initializeControllerFuture;

            // Construct the path where the image should be saved using the
            // pattern package.
            final path = join(
              // Store the picture in the temp directory.
              // Find the temp directory using the `path_provider` plugin.
              (await getTemporaryDirectory()).path,
              '${DateTime.now()}.png',
            );

            // Note: path is where the file exsits now.

            // Attempt to take a picture and log where it's been saved.
            await _controller.takePicture();

            // If the picture was taken, display it on a new screen.
            // Is the camera still active? ("on")?
            // Can the navigation stack be removed?
            print(path);

            // Questions / notes
            // Run the http post here. As bytes? With content disposition?
            // Read the file
            // Why can't the photo exist in memory, and not write to disk?
            // Too big of an object to be handling?
            // Remember, you're using the OS! iOS actually.

// Client:
            // - Open file

            // File file = File(path);
            // - Read file
            // Unsigned integer list
            // Uint8List fileBlob = await file.readAsBytes();

            // Is https required on IOS?! Shoot. How to get local https working?
            // What should the address be, if this is running on the phone?
            print("hello");

            var uri = Uri.parse('http://localhost:3001/img-upload');
            var request = http.MultipartRequest('POST', uri);

            // ignore: unused_local_variable
            var multipartFile =
                await http.MultipartFile.fromPath('FILE_NAME', path);

            // ignore: unused_local_variable
            var response = await request.send();

// Server:
            // Allow requests from different domains. CORS middleware.
            // Add back the image upload.
            // Test client and server independently if possible.

            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => DisplayPictureScreen(imagePath: path),
              ),
            );
          } catch (e) {
            // If an error occurs, log the error to the console.
            print(e);
          }
        },
      ),
    );
  }
}

// A widget that displays the picture taken by the user.
class DisplayPictureScreen extends StatelessWidget {
  final String imagePath;

  const DisplayPictureScreen({Key key, this.imagePath}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(title: Text('Display the Picture')),
        // The image is stored as a file on the device. Use the `Image.file`
        // constructor with the given path to display the image.
        body: InstructionBody(
          imagePath: imagePath,
        ));
  }
}

class InstructionBody extends StatelessWidget {
  final String imagePath;
  const InstructionBody({Key key, this.imagePath}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Center(
        child: GestureDetector(
          onTap: () {
            // Do something with tap
          },
          child: Container(
            color: Color.fromRGBO(240, 240, 240, 1.0),
            alignment: Alignment.topLeft,
            child: Padding(
              padding: const EdgeInsets.fromLTRB(10, 5, 10, 0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.max,
                children: [
                  Text(
                    'Chicken Chores 🐓',
                    style: TextStyle(
                        color: Color.fromRGBO(44, 44, 44, 1.0),
                        fontSize: 22,
                        fontWeight: FontWeight.bold),
                  ),
                  Text(
                    '⏱ 8m 32s  |  23 Steps',
                    style: TextStyle(
                      color: Color.fromRGBO(44, 44, 44, 1.0),
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.only(top: 6.0),
                    child: Row(
                      children: [
                        Text(
                          'Jesse Wright',
                          style: TextStyle(
                            color: Color.fromRGBO(0, 0, 0, 1.0),
                          ),
                        ),
                        const Icon(
                          FeatherIcons.chevronRight,
                          size: 20,
                        ),
                      ],
                    ),
                  ),
                  Container(
                    margin: EdgeInsets.all(10.0),
                    alignment: Alignment.topCenter,
                    child: ClipRRect(
                      borderRadius: BorderRadius.all(
                        Radius.circular(20),
                      ),
                      child: Image.network('https://picsum.photos/250?image=9'),
                    ),
                  ),
                  Container(
                    height: 50,
                    color: Color.fromRGBO(255, 255, 255, 1.0),
                  ),
                  Image.file(File(imagePath)),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
