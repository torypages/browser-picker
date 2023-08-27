# Browser Picker

## Requirements
* Multiple firefox browser profiles
* Each profile must have its own firefox-profilename.desktop file, e.x.:
```
t@tth:~/.local/share/applications$ cat firefox-t.desktop
[Desktop Entry]
Version=1.0
Name=[T] Firefox Web Browser
Comment=[T] Browse the World Wide Web
GenericName=Web Browser
Keywords=Internet;WWW;Browser;Web;Explorer
Exec=firefox --class [T]Firefox -P T %u
Terminal=false
X-MultipleArgs=false
Type=Application
Icon=/home/t/.local/share/applications/icons/t-firefox.svg
Categories=GNOME;GTK;Network;WebBrowser;
MimeType=text/html;text/xml;application/xhtml+xml;application/xml;application/rss+xml;application/rdf+xml;image/gif;image/jpeg;image/png;x-scheme-handler/http;x-scheme-handler/https;x-scheme-handler/ftp;x-scheme-handler/chrome;video/webm;application/x-xpinstall;
StartupNotify=true
Actions=new-window;new-private-window;

[Desktop Action new-window]
Name=Open a New Window
Exec=firefox --class [T]Firefox -new-window -P T %u

[Desktop Action new-private-window]
Name=Open a New Private Window
Exec=firefox --class [T]Firefox -private-window -P T
```
* set your OS to use this app as the web browser launcher
```
xdg-settings set default-web-browser browser-picker.desktop
```
* This app also needs a launcher
```
t@tth:~/.local/share/applications$ cat browser-picker.desktop
[Desktop Entry]
Version=1.0
Name=Browser Picker
Exec=browser-picker %u
Terminal=false
X-MultipleArgs=false
Type=Application
StartupNotify=true
MimeType=x-scheme-handler/unknown;x-scheme-handler/about;x-scheme-handler/https;x-scheme-handler/http;text/html;

```
