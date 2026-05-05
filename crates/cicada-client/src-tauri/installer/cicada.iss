; Inno Setup 脚本 — 知了 Cicada 安装程序
; 语言: 简体中文

#define MyAppName "知了"
#define MyAppNameEn "Cicada"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "SECTL"
#define MyAppURL "https://github.com/SECTL/Cicada"
#define MyAppExeName "cicada-client.exe"

[Setup]
AppId={{8B3F9A2D-5C1E-4E7A-B6D8-9F4C1E2A3D5B}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppNameEn}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=..\..\..\LICENSE
OutputDir=Output
OutputBaseFilename=Cicada_Setup
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
ArchitecturesInstallIn64BitMode=x64compatible

; 中文
[Languages]
Name: "chinese"; MessagesFile: "compiler:Languages\ChineseSimplified.isl"

[Tasks]
Name: "desktopicon"; Description: "创建桌面快捷方式"; GroupDescription: "附加图标:"

[Files]
Source: "build\cicada-client.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "build\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\卸载 {#MyAppName}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "启动 {#MyAppName}"; Flags: nowait postinstall skipifsilent
Filename: "{app}\{#MyAppExeName}"; Parameters: "--silent"; Flags: nowait runminimized unchecked; Description: "开机自启（静默启动到托盘）"

[UninstallDelete]
Type: filesandordirs; Name: "{localappdata}\Cicada"

[Code]
function InitializeSetup: Boolean;
begin
  Result := True;
end;
