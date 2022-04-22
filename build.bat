:: A convenience script for calling CMake.
rmdir /s /q build
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --config Release
cd ..
