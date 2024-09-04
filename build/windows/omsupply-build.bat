@ECHO ##### Removing previous builds #####
@rmdir "omSupply" /s /q

@ECHO ##### Starting omsupply builds #####
mkdir "omSupply"
mkdir "omSupply\Server"
mkdir "omSupply\Desktop"
xcopy "server\configuration" "omSupply\Server\configuration" /e /h /c /i

copy "server\server\omSupply.ico" "build\omSupply.ico"
xcopy "build\*.*" "omSupply" /c
xcopy "build\windows\*.*" "omSupply" /c
xcopy "build\windows\demo" "omSupply\demo" /c /y /i
copy "version.txt" "omSupply\version.txt"

start /wait /b build\windows\omsupply-prepare.bat
@if %errorlevel% neq 0 exit /b %errorlevel%

@ECHO ##### Building omsupply for sqlite #####
@cd server 
cargo build --release --bin omsupply_service && copy "target\release\omsupply_service.exe" "..\omSupply\Server\omSupply-sqlite.exe"
@if %errorlevel% neq 0 exit /b %errorlevel%

@ECHO ##### Building sqlite omsupply server #####
cd cargo build --release && copy "target\release\remote_server.exe" "..\omSupply\Server\omSupply-server-sqlite.exe"
@if %errorlevel% neq 0 exit /b %errorlevel%

@ECHO ##### Building omsupply for postgres #####
cargo build --release --bin omsupply_service --features postgres && copy "target\release\omsupply_service.exe" "..\omSupply\Server\omSupply-postgres.exe"
@if %errorlevel% neq 0 exit /b %errorlevel%

@ECHO ##### Building omSupply cli #####
cargo build --release --bin remote_server_cli && copy "target\release\remote_server_cli.exe" "..\omSupply\Server\omSupply-cli-sqlite.exe"
cargo build --release --bin remote_server_cli --features postgres && copy "target\release\remote_server_cli.exe" "..\omSupply\Server\omSupply-cli-postgres.exe"
@if %errorlevel% neq 0 exit /b %errorlevel%

@ECHO ##### Building connection test utility #####
cargo build --release --bin test_connection && copy "target\release\test_connection.exe" "..\omSupply\Server\test-connection-sqlite.exe"
cargo build --release --bin test_connection --features postgres && copy "target\release\test_connection.exe" "..\omSupply\Server\test-connection-postgres.exe"
@if %errorlevel% neq 0 exit /b %errorlevel%

@ECHO ##### Building omSupply for the desktop #####
@cd "..\client" 
yarn electron:build && xcopy "packages\electron\out\open mSupply-win32-x64\**" "..\omSupply\Desktop\" /e /h /c /i
@if %errorlevel% neq 0 exit /b %errorlevel%

@ECHO ##### Building omSupply apk #####
@REM copy the keystore and local.properties for apk signing
copy c:\android\local.properties packages\android\ /y && copy c:\android\release.keystore packages\android\app\ /y
yarn android:build:release
copy packages\android\app\build\outputs\apk\release\*.apk ..\omSupply
@if %errorlevel% neq 0 exit /b %errorlevel%
